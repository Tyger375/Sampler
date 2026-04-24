use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::graphics::event::GraphicsEvent;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::checkbox::UICheckBox;
use crate::graphics::ui::intinput::{IntInputConfig, UIIntInput};
use crate::graphics::ui::text::UIText;
use crate::navigator::{Navigator, NavigatorMessage};
use crate::settings::manager::SettingsManager;
use crate::settings_components::pads::PadsComponent;
use crate::settings_components::SettingsEvent;
use crate::utils::{int_to_note, CustomGraphicsEvent, MAX_MIDI_CHANNELS, MAX_MIDI_NOTE};

pub struct PadSettings {
    page_focus: i8,
    data: ScreenData,
    navigator: Navigator,
    pads_manager_paused: Arc<AtomicBool>,
    pads_manager_debug: Arc<AtomicBool>,
    settings_manager: Arc<SettingsManager<SettingsEvent>>
}

impl PadSettings {
    pub fn factory(
        navigator: Navigator,
        pads_manager_paused: Arc<AtomicBool>,
        pads_manager_debug: Arc<AtomicBool>,
        settings_manager: Arc<SettingsManager<SettingsEvent>>
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> + 'static {
        move |_| {
            Box::new(
                Self::new(
                    navigator.clone(),
                    pads_manager_paused.clone(),
                    pads_manager_debug.clone(),
                    settings_manager.clone()
                )
            )
        }
    }

    pub fn new(
        navigator: Navigator,
        pads_manager_paused: Arc<AtomicBool>,
        pads_manager_debug: Arc<AtomicBool>,
        settings_manager: Arc<SettingsManager<SettingsEvent>>
    ) -> Self {
        PadSettings {
            page_focus: -1,
            data: ScreenData::new(),
            navigator,
            pads_manager_paused,
            pads_manager_debug,
            settings_manager
        }
    }

    fn select_pad(&mut self) {
        self.page_focus = -1;
        self.data.row_offset = 0;
        self.data.clear();

        self.data.add_element(UIText::new("Press button".to_string()));

        let debug = self.pads_manager_debug.clone();
        self.data.add_element(
            UICheckBox::new(
                "Debug".to_string(),
                move |value| {
                    debug.clone().store(value, Ordering::Relaxed);
                    value
                },
                self.pads_manager_debug.load(Ordering::Relaxed)
            )
        );
    }

    fn pad_selected(&mut self) {
        let page_focus = self.page_focus;
        if page_focus < 0 || page_focus > 8 {
            return;
        }
        self.data.row_offset = 0;
        let config = self.settings_manager.get_component::<PadsComponent, _, _>("pads", |component| {
            component.get_data_config(page_focus as u8)
        }).unwrap();

        self.data.clear();
        self.data.add_element(
            UIText::new(format!("PAD: {}", page_focus + 1))
        );
        log::info!("CONFIG: {:?}", config);

        let settings_manager = self.settings_manager.clone();
        self.data.add_element(
            UIIntInput::new(
                IntInputConfig {
                    text: "Note".to_string(),
                    format_value: Box::new(|value| {
                        int_to_note(value)
                    }),
                    on_change: Box::new(|value, _| {
                        if value < 0 {
                            0
                        } else if value > MAX_MIDI_NOTE as i32 {
                            MAX_MIDI_NOTE as i32
                        } else {
                            value
                        }
                    }),
                    on_done: Box::new(move |value| {
                        settings_manager.clone().get_component::<PadsComponent, _, _>("pads", |component| {
                            component.set_pad_note(page_focus as u8, value as u8);
                        });
                    })
                }, config.note as i32
            )
        );

        let settings_manager = self.settings_manager.clone();
        self.data.add_element(
            UIIntInput::new(
                IntInputConfig {
                    text: "Channel".to_string(),
                    format_value: Box::new(|value| {
                        (value + 1).to_string()
                    }),
                    on_change: Box::new(|value, _| {
                        if value < 0 {
                            0
                        } else if value > MAX_MIDI_CHANNELS as i32 {
                            MAX_MIDI_CHANNELS as i32
                        } else {
                            value
                        }
                    }),
                    on_done: Box::new(move |value| {
                        settings_manager.clone().get_component::<PadsComponent, _, _>("pads", |component| {
                            component.set_pad_channel(page_focus as u8, value as u8);
                        });
                    })
                }, config.channel as i32
            )
        );

        let settings_manager = self.settings_manager.clone();
        self.data.add_element(
            UIIntInput::new(
                IntInputConfig {
                    text: "Threshold".to_string(),
                    format_value: Box::new(|value| {
                        value.to_string()
                    }),
                    on_change: Box::new(|value, _| {
                        if value < 50 {
                            50
                        } else if value > 1000 {
                            1000
                        } else {
                            value
                        }
                    }),
                    on_done: Box::new(move |value| {
                        settings_manager.clone().get_component("pads", |component: &PadsComponent| {
                            component.set_pad_threshold(page_focus as u8, value as u16);
                        });
                    })
                }, config.threshold as i32
            )
        );

        let settings_manager = self.settings_manager.clone();
        let navigator = self.navigator.clone();
        self.data.add_element(
            UIButton::new(
                "Save".to_string(),
                move || {
                    settings_manager.clone().get_component::<PadsComponent, _, _>("pads", |component| {
                        component.commit();
                    });
                    navigator.clone().send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Back)).ok();
                }
            )
        )
    }
}

impl Screen for PadSettings {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }

    fn on_start(&mut self) {
        self.data.row_offset = 0;
        for elem in self.data.elements.iter_mut() {
            elem.on_event(GraphicsEvent::ScreenStart);
        }

        self.pads_manager_paused.store(true, Ordering::Relaxed);

        self.select_pad();
    }

    fn on_end(&mut self) {
        for elem in self.get_data_mut().elements.iter_mut() {
            elem.on_event(GraphicsEvent::ScreenEnd);
        }

        self.pads_manager_paused.store(false, Ordering::Relaxed);
    }

    fn on_back(&mut self) -> bool {
        if self.page_focus >= 0 && !self.data.focus {
            self.select_pad();
            return true;
        }

        let data = self.get_data_mut();
        if data.focus {
            data.focus = false;
            return data.elements[data.row_offset].on_event(GraphicsEvent::Back);
        }
        false
    }

    fn on_custom_event(&mut self, event: u32) -> bool {
        let event: CustomGraphicsEvent = event.into();

        log::info!("Custom event: {} {}", event.is_long_click(), event.get_channel());
        if !event.is_long_click() && !event.is_shortcut() && self.page_focus < 0 {
            self.page_focus = event.get_channel() as i8;
            self.pad_selected();
            return true;
        }

        false
    }
}