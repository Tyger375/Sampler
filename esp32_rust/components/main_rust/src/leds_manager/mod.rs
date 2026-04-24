use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Sender;
use esp_idf_svc::hal::i2c::I2cDriver;
use crate::spawn_task;

enum LedsMessage {
    SetSequencer(u8, bool),
    Set(u8, bool)
}

pub struct LedsManager {
    tx: Sender<LedsMessage>
}

impl LedsManager {
    pub fn new(
        i2c: Arc<Mutex<I2cDriver<'static>>>,
        addr: u8
    ) -> Self {
        let (tx, rx) = mpsc::channel::<LedsMessage>();
        let manager = Self {
            tx
        };

        let i2c = i2c.clone();
        spawn_task!({
            name: "leds_manager",
            stack_size: 2048,
            priority: 10,
        }, move || {
            // LEDs: setting I2C extender PINs to OUTPUT
            {
                let mut guard = i2c.lock().unwrap();
                guard.write(addr, &[0x06, 0x00], 1000).unwrap();

                guard.write(addr, &[0x02, 0], 1000).unwrap();
            }

            let mut leds = 0u8;

            let mut seq_status = 0u8;
            let mut seq_enabled = false;

            loop {
                if let Ok(message) = rx.recv() {
                    match message {
                        LedsMessage::SetSequencer(status, enabled) => {
                            seq_status = status;
                            seq_enabled = enabled;
                        },
                        LedsMessage::Set(index, press) => {
                            if press {
                                leds |= 1 << index;
                            } else {
                                leds &= !(1 << index);
                            }
                        }
                    }

                    let mut status = 0u8;
                    if seq_enabled {
                        status |= seq_status;
                    }
                    status |= leds;

                    let mut guard = i2c.lock().unwrap();
                    guard.write(0x20, &[0x02, status], 1000).ok();
                }
            }
        });

        manager
    }

    pub fn set_led(&self, index: u8, press: bool) {
        self.tx.send(LedsMessage::Set(index, press)).ok();
    }

    pub fn set_sequencer(&self, sequencer: u8, enabled: bool) {
        self.tx.send(LedsMessage::SetSequencer(sequencer, enabled)).ok();
    }
}