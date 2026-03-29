mod task;

use crate::ads1015::FsrConfig::Fsr6_144;
use crate::ads1015::{
    ADS1015Config, CompLatchingConfig, CompModeConfig, CompPolarityConfig, CompQueueDisableConfig,
    DataRateConfig, MuxConfig, OpModeConfig, ADS1015,
};
use crate::midi::MidiType;
use crate::pads::task::{TaskState, TaskStatus};
use crate::{delay_us, spawn_task, timestamp};
use esp_idf_svc::hal::cpu::Core::Core0;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2c::{I2c, I2cConfig, I2cDriver};
use esp_idf_svc::hal::task::queue::Queue;
use esp_idf_svc::hal::units::Hertz;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{array, thread};

#[derive(Debug, Copy, Clone)]
pub struct PadInputEvent {
    pub index: u8,
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
    pub midi_type: MidiType,
}

#[derive(Copy, Clone)]
pub enum PadState {
    Idle,
    Attack,
    Sustain,
    Release,
}

#[derive(Copy, Clone)]
pub enum PadPressType {
    OneShot,
}

#[derive(Copy, Clone)]
pub struct DrumPad {
    // MIDI settings
    pub note: u8,
    pub channel: u8,
    // internal settings
    pub press_type: PadPressType,
    pub threshold: u16,
    // states
    state: PadState,
    peak: u16,
    timer_start: u32,
}

impl DrumPad {
    fn new(note: u8, channel: u8, press_type: PadPressType, threshold: u16) -> Self {
        DrumPad {
            note,
            channel,
            press_type,
            threshold,
            state: PadState::Idle,
            peak: 0,
            timer_start: 0,
        }
    }
}

fn process_pad_physics(index: u8, settings: &mut DrumPad, value: u16) -> Option<PadInputEvent> {
    let now = timestamp();

    match settings.state {
        PadState::Idle => {
            if value > settings.threshold {
                settings.state = PadState::Attack;
                settings.peak = value;
                settings.timer_start = now;
            }
            None
        }
        PadState::Attack => {
            // Update peak if sample is higher
            if value > settings.peak {
                settings.peak = value;
            }

            // Wait 5ms
            if now - settings.timer_start >= 5 {
                let velocity = if settings.peak > 2047 {
                    127
                } else {
                    settings.peak >> 4
                };

                settings.state = PadState::Sustain;

                Some(PadInputEvent {
                    index,
                    channel: settings.channel,
                    note: settings.note,
                    velocity: velocity as u8,
                    midi_type: MidiType::NoteOn,
                })
            } else {
                None
            }
        }
        PadState::Sustain => {
            // Look for the value to drop below threshold
            if value < (settings.threshold as f32 * 0.8) as u16 {
                // 20% Hysteresis
                settings.state = PadState::Release;
                settings.timer_start = now;
            }

            None
        }
        PadState::Release => {
            settings.state = PadState::Idle;

            Some(PadInputEvent {
                index,
                channel: settings.channel,
                note: settings.note,
                velocity: 0,
                midi_type: MidiType::NoteOff,
            })
        }
    }
}

pub struct PadsManager {
    settings: Arc<Mutex<[DrumPad; 8]>>,
    task_status: Arc<TaskState>,
    pads_midi_events: Arc<Queue<PadInputEvent>>,
}

impl PadsManager {
    pub fn new<I2C: I2c + 'static>(
        i2c: I2C,
        sda: AnyIOPin<'static>,
        scl: AnyIOPin<'static>,
        addr1: u8,
        addr2: u8,
    ) -> Result<Self, anyhow::Error> {
        let i2c_config = I2cConfig::new().baudrate(Hertz(400_000));
        let i2c_master = I2cDriver::new(i2c, sda, scl, &i2c_config)?;

        let i2c_bus = Arc::new(Mutex::new(i2c_master));

        let ads_cfg = ADS1015Config {
            mux_config: MuxConfig::MUX0,
            fsr_mode: Fsr6_144,
            op_mode: OpModeConfig::Continuous,
            data_rate: DataRateConfig::DataRate3300,
            comparator_mode: CompModeConfig::Traditional,
            comparator_polarity: CompPolarityConfig::ActiveLow,
            comparator_latching: CompLatchingConfig::NonLatching,
            queue_and_disable: CompQueueDisableConfig::DisableComp,
        };

        let ads1 = ADS1015::new(i2c_bus.clone(), addr1);
        let ads2 = ADS1015::new(i2c_bus.clone(), addr2);

        ads1.set_config(&ads_cfg)?;
        ads2.set_config(&ads_cfg)?;

        let settings = Arc::new(Mutex::new(array::from_fn(|i| {
            DrumPad::new(60 + i as u8, 0, PadPressType::OneShot, 50)
        })));
        let queue = Arc::new(Queue::new(64));
        let task_status = Arc::new(TaskState::new(TaskStatus::Running));

        {
            let queue = queue.clone();
            let task_status = task_status.clone();
            let settings = settings.clone();

            let _handle = spawn_task!({
                name: "pads_input_task",
                stack_size: 4096,
                priority: 15,
                pin_to_core: Some(Core0),
            }, move || {
                let mut pads_settings = {
                    let guard = settings.lock().unwrap();
                    *guard
                };

                let mut channel: usize = 0;

                loop {
                    match task_status.get() {
                        TaskStatus::Running => {
                            delay_us(500);

                            let value1 = ads1.read();
                            let value2 = ads2.read();

                            if let Some(item) = process_pad_physics(channel as u8, &mut pads_settings[channel], value1) {
                                queue.send_back(item, 0).unwrap();
                            }
                            if let Some(item) = process_pad_physics((channel as u8) + 4, &mut pads_settings[channel + 4], value2) {
                                queue.send_back(item, 0).unwrap();
                            }

                            channel = (channel + 1) % 4;

                            let cfg = MuxConfig::mux_from(channel as u8);
                            ads1.set_mux(&cfg).unwrap();
                            ads2.set_mux(&cfg).unwrap();
                        }
                        TaskStatus::Suspended => {
                            thread::sleep(Duration::from_secs(1));
                        }
                        TaskStatus::Updating => {
                            let guard = settings.lock().unwrap();
                            pads_settings = *guard;
                            task_status.set(TaskStatus::Running);
                        }
                    }
                }
            });
        }

        Ok(PadsManager {
            settings,
            task_status,
            pads_midi_events: queue,
        })
    }

    pub fn get_pad_settings(&self) -> Arc<Mutex<[DrumPad; 8]>> {
        self.settings.clone()
    }

    pub fn commit_pad_settings(&self) {
        self.task_status.set(TaskStatus::Updating);
    }

    pub fn get_midi_events(&self) -> Arc<Queue<PadInputEvent>> {
        self.pads_midi_events.clone()
    }
}
