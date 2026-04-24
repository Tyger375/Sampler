use crate::graphics::drivers::lcd1602::Lcd1602;
use crate::graphics::event::GraphicsEvent;
use crate::graphics::manager::GraphicsManager;
use crate::midi::{MidiType, MIDI};
use crate::navigator::NavigatorMessage;
use crate::pads::{PadButtonEvent, PadInputEventType, PadsManager};
use crate::quantizer::Quantizer;
use crate::screens::home::HomeScreen;
use crate::screens::pad_settings::PadSettings;
use crate::screens::settings::SettingsScreen;
use crate::selector::{RotationEvent, Selector, SelectorEvent};
use crate::sequencer::Sequencer;
use crate::settings::manager::SettingsManager;
use crate::settings_components::config::ConfigComponent;
use crate::settings_components::pads::PadsComponent;
use crate::settings_components::shortcuts::{Shortcut, ShortcutsComponent};
use crate::settings_components::SettingsEvent;
use crate::utils::{log_main_stack, timestamp, CustomGraphicsEvent};
use crate::vendor::Vendor;
use core::default::Default;
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::task::notification::Notification;
use esp_idf_svc::hal::units::Hertz;
use esp_idf_svc::sys::{
    eNotifyAction_eIncrement, tskTaskControlBlock, ulTaskGenericNotifyTake, xTaskGenericNotify,
    xTaskGetCurrentTaskHandle,
};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{ptr, thread};
use crate::leds_manager::LedsManager;
use crate::screens::sequencer::SequencerScreen;
use crate::screens::sequencer_project::SequencerProjectScreen;
use crate::screens::sequencer_tracks::SequencerTracksScreen;
use crate::settings_components::sequencer::SequencerSettings;
use crate::settings_components::sequencer_projects::SequencerProjects;

pub mod ads1015;
pub mod graphics;
pub mod midi;
pub mod navigator;
pub mod pads;
pub mod quantizer;
pub mod screens;
pub mod selector;
mod sequencer;
pub mod settings;
pub mod settings_components;
pub mod task;
pub mod utils;
pub mod vendor;
pub mod leds_manager;

enum SequencerMessage {
    NoteOn(u8),
    NoteOff(u8),
}

static VENDOR_TASK_H: AtomicPtr<tskTaskControlBlock> = AtomicPtr::new(ptr::null_mut());

#[no_mangle]
extern "C" fn tud_vendor_rx_cb(_itf: u8) {
    let handle = VENDOR_TASK_H.load(Ordering::Relaxed);
    if !handle.is_null() {
        unsafe {
            xTaskGenericNotify(handle, 0, 0, eNotifyAction_eIncrement, ptr::null_mut());
        }
    }
}

#[no_mangle]
extern "C" fn rust_main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    let (quantizer_tx, quantizer_rx) = mpsc::channel::<u8>();
    let pads_manager = PadsManager::new(
        peripherals.i2c0,
        peripherals.pins.gpio12.into(),
        peripherals.pins.gpio11.into(),
        0x48,
        0x49,
    )
    .expect("Failed to create PadsManager");
    let pads_manager = Arc::new(pads_manager);

    // Settings
    let (settings_manager, settings_rx, littlefs) =
        SettingsManager::new().expect("Failed to create SettingsManager");
    core::mem::forget(littlefs);
    let settings_manager: Arc<SettingsManager<SettingsEvent>> = Arc::new(settings_manager);

    log_main_stack("After adding components");

    settings_manager.add_component("config", |tx| ConfigComponent::new(tx));
    settings_manager.add_component("pads", |tx| PadsComponent::new(tx));
    settings_manager.add_component("shortcuts", |tx| ShortcutsComponent::new(tx));
    settings_manager.add_component("sequencer", |tx| SequencerSettings::new(tx));
    settings_manager.add_component("sequencer_projects", |tx| SequencerProjects::new(tx));

    log_main_stack("After adding components");

    {
        let settings = settings_manager.clone();
        let pads_manager = pads_manager.clone();
        spawn_task!({
            name: "settings",
            stack_size: 4096,
            priority: 6,
        }, move || loop {
            if let Ok(item) = settings_rx.recv() {
                match item {
                    SettingsEvent::ConfigBpm => {
                        settings.get_component("config", |component: &ConfigComponent| {
                            component.save();
                            let bpm = component.bpm();
                            log::info!("Setting new bpm: {}", bpm);
                            quantizer_tx.send(bpm).unwrap();
                        });
                    }
                    SettingsEvent::PadConfig => {
                        settings.get_component::<PadsComponent, _, _>("pads", |component| {
                            component.save();
                            pads_manager.request_update_settings(&component.get_configs());
                        });
                    }
                }
            }
        });
    }

    settings_manager.get_component::<PadsComponent, _, _>("pads", |component| {
        pads_manager.request_update_settings(&component.get_configs());
    });

    // Vendor
    {
        let settings = settings_manager.clone();
        spawn_task!({
            name: "vendor_rx",
            stack_size: 4096,
            priority: 4,
        }, move || {
            let current_handle = unsafe { xTaskGetCurrentTaskHandle() };
            VENDOR_TASK_H.store(current_handle, Ordering::SeqCst);

            let mut message = String::new();

            loop {
                unsafe {
                    ulTaskGenericNotifyTake(0, 1, delay::BLOCK);
                }

                while Vendor::available() > 0 {
                    let chunk = Vendor::read(64);
                    if chunk.is_empty() {
                        continue;
                    }

                    message.push_str(&chunk);
                    while let Some(pos) = message.find("\n") {
                        let cmd: String = message.drain(..pos + 1).collect();
                        let cmd = cmd.trim();

                        if cmd.is_empty() {
                            continue;
                        }

                        log::info!("Vendor Received: {}", cmd);

                        let results: Vec<&str> = cmd
                            .split_whitespace()
                            .map(|s| s)
                            .collect();

                        if !results.is_empty() {
                            // On Vendor CMD
                            match results[0] {
                                "READ_CONFIG" => {
                                    if let Some(component_name) = results.get(1) {
                                        let args = results.get(2..).unwrap_or(&[]);
                                        let result = settings.direct_read(component_name, &Vec::from(args));
                                        Vendor::respond(result);
                                    } else {
                                        Vendor::respond("invalid args".to_string());
                                    }
                                }
                                _ => {
                                    log::warn!("Unknown USB command: {}", results[0]);
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    let config = I2cConfig::new().baudrate(Hertz(400_000));
    let i2c_master = I2cDriver::new(
        peripherals.i2c1,
        peripherals.pins.gpio21,
        peripherals.pins.gpio18,
        &config,
    )
    .unwrap();
    let i2c_master = Arc::new(Mutex::new(i2c_master));

    // LEDs manager
    let leds_manager = Arc::new(LedsManager::new(i2c_master.clone(), 0x20));

    // Pads manager
    let (pads_tx, pads_rx) = mpsc::channel();
    let pads_midi_paused = Arc::new(AtomicBool::new(false));

    {
        let leds_manager = leds_manager.clone();
        let queue = pads_manager.pads_midi_events.clone();
        let paused = pads_midi_paused.clone();
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

                            pads_tx.send(PadButtonEvent {
                                index: packet.index,
                                pressed: if let MidiType::NoteOn = midi_type { true } else { false }
                            }).ok();

                            if !paused.load(Ordering::Relaxed) {
                                let bytes: [u8; 4] = [midi.0, midi.1 | (packet.channel & 0x0F), packet.note, packet.velocity];
                                MIDI::send(&bytes);
                            }

                            if let MidiType::NoteOn = midi_type {
                                leds_manager.set_led(packet.index, true);
                            } else if let MidiType::NoteOff = midi_type {
                                leds_manager.set_led(packet.index, false);
                            }
                        }
                        PadInputEventType::Debug => {
                            let value = (packet.note as u16) | ((packet.velocity as u16) << 8);
                            Vendor::write_raw(&format!("{}: ({}, {});", packet.index, value, packet.channel));
                            //Vendor::flush();
                        }
                    }
                };
            }
        });
    }

    // Sequencer
    let (sequencer_tx, sequencer_channel) = mpsc::channel::<SequencerMessage>();
    let sequencer = Arc::new(Sequencer::new());

    /*
    sequencer.new_project("Project".to_string(), 1);
    sequencer.add_track();
    sequencer.edit_track(0, |track| {
        track.resolution = SequencerResolution::Beat;
        track.triggers.push(0);
        track.triggers.push(1);
        track.triggers.push(2);
        track.triggers.push(3);
    });
    */

    {
        let sequencer = sequencer.clone();
        let _handle = spawn_task!({
            name: "sequencer_task",
            stack_size: 4096,
            priority: 23,
            pin_to_core: Some(Core::Core1),
        }, move || {
            loop {
                if let Ok(item) = sequencer_channel.recv() {
                    if sequencer.enabled() {
                        match item {
                            SequencerMessage::NoteOn(step) => {
                                sequencer.step_trigger_on(step);
                            }
                            SequencerMessage::NoteOff(step) => {
                                sequencer.step_trigger_off(step);
                            }
                        }
                    }
                }
            }
        });
    }

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
                    log::info!("Starting quantizer with bpm: {item}");
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
    )
    .expect("Failed to create Selector");
    let selector_queue = selector.get_queue();

    // Graphics
    let mut graphics_manager = GraphicsManager::new();
    graphics_manager.install_driver(Lcd1602::new(i2c_master.clone(), 0x27));
    let (navigator, navigator_rx) = mpsc::channel::<NavigatorMessage>();

    graphics_manager.load_screen("home", HomeScreen::factory(navigator.clone()));
    graphics_manager.load_screen(
        "settings",
        SettingsScreen::factory(
            navigator.clone(),
            settings_manager.clone()
        ),
    );
    graphics_manager.load_screen(
        "pad_settings",
        PadSettings::factory(
            navigator.clone(),
            pads_midi_paused.clone(),
            pads_manager.is_debug(),
            settings_manager.clone(),
        ),
    );
    graphics_manager.load_screen(
        "sequencer",
        SequencerScreen::factory(
            navigator.clone(),
            sequencer.clone(),
            settings_manager.clone()
        ),
    );
    graphics_manager.load_screen(
        "sequencer_project",
        SequencerProjectScreen::factory(
            navigator.clone(),
            sequencer.clone(),
            settings_manager.clone(),
        ),
    );
    graphics_manager.load_screen(
        "sequencer_tracks",
        SequencerTracksScreen::factory(
            navigator.clone(),
            sequencer.clone(),
            leds_manager.clone()
        ),
    );

    graphics_manager.navigate("home", vec![]);

    graphics_manager.update(false);

    let mut was_shortcut = false;
    let mut press_start_time = 0u32;
    let mut pads_press_start_times = [0u32; 8];
    const LONG_PRESS_THRESHOLD_MS: u32 = 500;

    loop {
        let mut needs_refresh: bool = false;

        if let Ok(message) = navigator_rx.try_recv() {
            match message {
                NavigatorMessage::Navigate(route) => graphics_manager.navigate(&route, vec![]),
                NavigatorMessage::Back => graphics_manager.navigate_back(),
                NavigatorMessage::GraphicsEvent(event) => graphics_manager.send_event(event),
                NavigatorMessage::CustomEvent(event) => { graphics_manager.send_custom_event(event); },
            }
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
                        if press_start_time == 0 {
                            press_start_time = timestamp();
                        } else {
                            press_start_time = 0; // Cancel press
                        }
                        needs_refresh = true;
                    } else {
                        if was_shortcut {
                            was_shortcut = false;
                            needs_refresh = true;
                            press_start_time = 0;
                        } else {
                            let duration = timestamp() - press_start_time;

                            let event = if duration >= LONG_PRESS_THRESHOLD_MS {
                                GraphicsEvent::Back
                            } else {
                                GraphicsEvent::Click
                            };
                            graphics_manager.send_event(event);
                            press_start_time = 0;
                            needs_refresh = true;
                        }
                    }
                }
            }
        } else if let Ok(event) = pads_rx.try_recv() {
            if pads_manager.is_debug.load(Ordering::Relaxed) {
                continue;
            }
            if event.pressed {
                pads_press_start_times[event.index as usize] = timestamp();
            } else {
                let mut custom_event = CustomGraphicsEvent::new().with_channel(event.index);
                let now = timestamp();
                let duration = now - pads_press_start_times[event.index as usize];
                if press_start_time != 0 && (now - press_start_time) >= LONG_PRESS_THRESHOLD_MS {
                    // Shortcut
                    custom_event.set_shortcut(true);
                    was_shortcut = true;
                } else if duration >= LONG_PRESS_THRESHOLD_MS {
                    custom_event.set_long_click(true)
                }

                let data: u32 = custom_event.into();
                //log::info!("custom event: {:b}", data);
                let shortcut = settings_manager
                    .get_component::<ShortcutsComponent, _, _>("shortcuts", |component| {
                        component.from_cevent(custom_event)
                    })
                    .expect("Couldn't find shortcuts component");
                if let Some(shortcut) = shortcut {
                    match shortcut {
                        Shortcut::NavigateScreen(screen) => {
                            graphics_manager.navigate(screen.as_str(), vec![]);
                            needs_refresh = true;
                        }
                    }
                } else {
                    needs_refresh = graphics_manager.send_custom_event(data);
                }
            }
        }

        if needs_refresh {
            graphics_manager.update(press_start_time != 0);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

/*CLion complains*/
#[allow(dead_code)]
fn main() {}
