use crate::graphics::drivers::lcd1602::Lcd1602;
use crate::graphics::event::GraphicsEvent;
use crate::graphics::manager::GraphicsManager;
use crate::midi::{MidiType, MIDI};
use crate::pads::{PadInputEventType, PadsManager};
use crate::quantizer::Quantizer;
use crate::screens::home::HomeScreen;
use crate::screens::settings::SettingsScreen;
use crate::selector::{RotationEvent, Selector, SelectorEvent};
use crate::settings::components::config::ConfigComponent;
use crate::settings::manager::SettingsManager;
use crate::utils::log_main_stack;
use crate::vendor::Vendor;
use core::default::Default;
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::task::notification::Notification;
use esp_idf_svc::hal::units::Hertz;
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub mod ads1015;
pub mod graphics;
pub mod midi;
pub mod pads;
pub mod quantizer;
pub mod screens;
pub mod selector;
pub mod settings;
pub mod task;
pub mod utils;
pub mod vendor;

enum SequencerMessage {
    NoteOn(u8),
    NoteOff(u8),
}

#[no_mangle]
extern "C" fn rust_main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let (quantizer_tx, quantizer_rx) = mpsc::channel::<u8>();

    // Settings
    let (settings_manager, settings_rx, littlefs) =
        SettingsManager::new().expect("Failed to create SettingsManager");
    core::mem::forget(littlefs);
    let settings_manager: Arc<SettingsManager> = Arc::new(settings_manager);

    settings_manager.add_component("config", |tx| ConfigComponent::new(tx));

    log_main_stack("After add_component");

    {
        let settings = settings_manager.clone();
        thread::spawn(move || loop {
            if let Ok(item) = settings_rx.recv() {
                if item == "config_bpm" {
                    settings.get_component("config", |component: &ConfigComponent| {
                        component.save();
                        let bpm = component.bpm();
                        log::info!("Setting new bpm: {}", bpm);
                        quantizer_tx.send(bpm).unwrap();
                    });
                }
            }
        });
    }

    let peripherals = Peripherals::take().unwrap();

    let config = I2cConfig::new().baudrate(Hertz(400_000));
    let i2c_master = I2cDriver::new(
        peripherals.i2c1,
        peripherals.pins.gpio21,
        peripherals.pins.gpio18,
        &config,
    ).unwrap();
    let i2c_master = Arc::new(Mutex::new(i2c_master));

    // LEDs manager
    let (led_tx, led_rx) = mpsc::channel::<(u8, bool)>();
    let led_i2c = i2c_master.clone();

    thread::spawn(move || {
        // LEDs: setting I2C extender PINs to OUTPUT
        let mut leds = 0u8;
        {
            let mut guard = led_i2c.lock().unwrap();
            guard.write(0x20, &[0x06, 0x00], 1000).unwrap();

            guard.write(0x20, &[0x02, leds], 1000).unwrap();
        }

        loop {
            if let Ok((index, press)) = led_rx.recv() {
                if press {
                    leds |= 1 << index;
                } else {
                    leds &= !(1 << index);
                }

                let mut guard = led_i2c.lock().unwrap();
                guard.write(0x20, &[0x02, leds], 1000).unwrap();
            }
        }
    });

    // Pads manager
    let pads_manager = PadsManager::new(
        peripherals.i2c0,
        peripherals.pins.gpio12.into(),
        peripherals.pins.gpio11.into(),
        0x48,
        0x49,
    ).expect("Failed to create PadsManager");

    {
        let leds_tx = led_tx.clone();
        let queue = pads_manager.get_midi_events();
        let _handle = spawn_task!({
            name: "pads_input_task",
            stack_size: 4096,
            priority: 23,
            pin_to_core: Some(Core::Core1),
        }, move || {
            loop {
                if let Some((packet, _)) = queue.recv_front(delay::BLOCK) {
                    match packet.event_type {
                        PadInputEventType::MIDI(midi_type) => {
                            let midi: (u8, u8) = midi_type.into();

                            let bytes: [u8; 4] = [midi.0, midi.1 | (packet.channel & 0x0F), packet.note, packet.velocity];
                            MIDI::send(&bytes);

                            if let MidiType::NoteOn = midi_type {
                                leds_tx.send((packet.index, true)).ok();
                            } else if let MidiType::NoteOff = midi_type {
                                leds_tx.send((packet.index, false)).ok();
                            }
                        }
                        PadInputEventType::Debug => {
                            let value = (packet.note as u16) | ((packet.velocity as u16) << 8);
                            Vendor::write(&format!("{}: {}", packet.index, value));
                            Vendor::flush();
                        }
                    }
                };
            }
        });
    }

    // Sequencer
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
                    SequencerMessage::NoteOn(_step) => {
                        /*if (step % 4) == 0 {
                            MIDI::send(&[0x09, 0x90, 70, 127]);
                        }*/
                    }
                    SequencerMessage::NoteOff(_step) => {
                        /*if (step % 4) == 0 {
                            MIDI::send(&[0x08, 0x80, 70, 0]);
                        }*/
                    }
                }
            }
        }
    });

    // Quantizer
    {
        let quantizer_seq_tx = sequencer_tx.clone();
        let settings = settings_manager.clone();
        let _handle = spawn_task!({
            name: "quantizer_task",
            stack_size: 4096,
            priority: 24,
            pin_to_core: Some(Core::Core1),
        }, move || {
            let notification = Notification::new();
            let quantizer = Arc::new(Quantizer::new(notification.notifier()).expect("Failed to create quantizer"));
            settings.get_component::<ConfigComponent, _, _>("config", |component| {
                quantizer.start(component.bpm()).unwrap();
            });

            const MIDI_SYNC_MSG: [u8; 4] = [0x0F, 0xF8, 0x00, 0x00];

            loop {
                if let Ok(item) = quantizer_rx.try_recv() {
                    quantizer.start(item).unwrap();
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
    }

    let selector = Selector::new(
        peripherals.pins.gpio8.into(),
        peripherals.pins.gpio7.into(),
        peripherals.pins.gpio9.into(),
    ).expect("Failed to create Selector");
    let selector_queue = selector.get_queue();

    // Graphics
    let mut graphics_manager = GraphicsManager::new();
    graphics_manager.install_driver(Lcd1602::new(i2c_master.clone(), 0x27));
    let (navigator, navigator_rx) = mpsc::channel::<String>();

    graphics_manager.load_screen("home", HomeScreen::factory(navigator.clone()));
    graphics_manager.load_screen(
        "settings",
        SettingsScreen::factory(settings_manager.clone()),
    );

    graphics_manager.navigate("home");

    graphics_manager.update();

    loop {
        let mut needs_refresh: bool = false;

        if let Ok(route) = navigator_rx.try_recv() {
            graphics_manager.navigate(&route);
            needs_refresh = true;
        } else if let Some((res, _)) = selector_queue.recv_front(delay::NON_BLOCK) {
            match res {
                SelectorEvent::Rotation(direction) => {
                    graphics_manager.send_event(if let RotationEvent::Left = direction {
                        GraphicsEvent::ScrollLeft
                    } else {
                        GraphicsEvent::ScrollRight
                    });
                    needs_refresh = true;
                }
                SelectorEvent::Click(press) => {
                    if press {
                        graphics_manager.send_event(GraphicsEvent::Click);
                        needs_refresh = true;
                    }
                }
            }
        }

        if needs_refresh {
            graphics_manager.update();
        }
        thread::sleep(Duration::from_millis(100));
    }
}

/*CLion complains*/
#[allow(dead_code)]
fn main() {}
