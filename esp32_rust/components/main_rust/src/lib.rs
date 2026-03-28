use crate::midi::MIDI;
use crate::pads::{PadsManager};
use crate::quantizer::Quantizer;
use core::default::Default;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::thread;
use std::thread::current;
use std::time::Duration;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::task::notification::{Notification, Notifier};
use esp_idf_svc::hal::units::Hertz;

pub mod ads1015;
pub mod midi;
pub mod pads;
pub mod quantizer;
pub mod task;

extern "C" {
    fn esp_delay_us(micros: u32);
    fn log_timestamp() -> u32;
}

pub fn delay_us(micros: u32) {
    unsafe {
        esp_delay_us(micros);
    }
}

pub fn timestamp() -> u32 {
    unsafe { log_timestamp() }
}

enum SequencerMessage {
    NoteOn(u8),
    NoteOff(u8)
}

#[no_mangle]
extern "C" fn rust_main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let config = I2cConfig::new().baudrate(Hertz(400_000));
    let mut i2c_master = I2cDriver::new(peripherals.i2c1, peripherals.pins.gpio21, peripherals.pins.gpio18, &config).unwrap();

    let led = mpsc::channel::<(u8, bool)>();

    let bytes = [0x06, 0x00];
    i2c_master.write(0x20, &bytes, 1000).unwrap();

    let pads_manager = PadsManager::new(
        peripherals.i2c0,
        peripherals.pins.gpio12.into(),
        peripherals.pins.gpio11.into(),
        0x48,
        0x49,
    ).expect("Failed to create PadsManager");

    let queue = pads_manager.get_midi_events();
    let _handle = spawn_task!({
        name: "pads_input_task",
        stack_size: 4096,
        priority: 23,
        pin_to_core: Some(Core::Core1),
    }, move || {
        loop {
            if let Some((packet, _)) = queue.recv_front(delay::BLOCK) {
                let midi: (u8, u8) = packet.midi_type.into();

                let bytes: [u8; 4] = [midi.0, midi.1 | (packet.channel & 0x0F), packet.note, packet.velocity];
                MIDI::send(&bytes);
            };
        }
    });

    let (sequencer_tx, sequencer_channel) = mpsc::channel::<SequencerMessage>();

    let _handle = spawn_task!({
        name: "sequencer_task",
        stack_size: 4096,
        priority: 23,
        pin_to_core: Some(Core::Core1),
    }, move || {
        loop {
            if let Ok(item) = sequencer_channel.recv() {
                match item {
                    SequencerMessage::NoteOn(step) => {
                        /*if (step % 4) == 0 {
                            MIDI::send(&[0x09, 0x90, 70, 127]);
                        }*/
                    }
                    SequencerMessage::NoteOff(step) => {
                        /*if (step % 4) == 0 {
                            MIDI::send(&[0x08, 0x80, 70, 0]);
                        }*/
                    }
                }
            }
        }
    });

    let paused = Arc::new(AtomicBool::new(false));
    let quantizer_seq_tx = sequencer_tx.clone();
    let _handle = spawn_task!({
        name: "quantizer_task",
        stack_size: 4096,
        priority: 24,
        pin_to_core: Some(Core::Core1),
    }, move || {
        let notification = Notification::new();
        let quantizer = Arc::new(Quantizer::new(notification.notifier()).expect("Failed to create quantizer"));
        quantizer.start(140).unwrap();

        const MIDI_SYNC_MSG: [u8; 4] = [0x0F, 0xF8, 0x00, 0x00];

        loop {
            if paused.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            notification.wait_any();
            MIDI::send(&MIDI_SYNC_MSG);

            let ticks = quantizer.ticks.load(Ordering::SeqCst);
            let steps = quantizer.steps.load(Ordering::SeqCst);

            if ticks == 0 {
                quantizer_seq_tx.send(SequencerMessage::NoteOn(steps)).ok();
            } else if ticks == 5 {
                quantizer_seq_tx.send(SequencerMessage::NoteOff(steps)).ok();
            }
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

/*CLion complains*/
#[allow(dead_code)]
fn main() {}