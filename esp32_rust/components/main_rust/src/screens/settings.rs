use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::intinput::{IntInputConfig, UIIntInput};
use crate::graphics::ui::text::UIText;
use crate::navigator::{Navigator, NavigatorMessage};
use crate::settings::manager::SettingsManager;
use crate::settings_components::config::ConfigComponent;
use crate::settings_components::SettingsEvent;

pub struct SettingsScreen {
    data: ScreenData
}

impl SettingsScreen {
    pub fn factory(
        navigator: Navigator,
        settings: Arc<SettingsManager<SettingsEvent>>
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> {
        move |_| Box::new(SettingsScreen::new(navigator.clone(), settings.clone()))
    }

    pub fn new(
        navigator: Navigator,
        settings: Arc<SettingsManager<SettingsEvent>>
    ) -> Self {
        let mut data = ScreenData::new();

        data.add_element(UIText::new("Settings".to_string()));

        let bpm = settings.get_component("config", |component: &ConfigComponent| {
            component.bpm()
        }).unwrap_or(140);

        let s = settings.clone();
        data.add_element(UIIntInput::new(
            IntInputConfig {
                text: "BPM".to_string(),
                format_value: Box::new(|value| {
                    value.to_string()
                }),
                on_change: Box::new(|value| {
                    if value > 200 {
                        200
                    } else if value < 60 {
                        60
                    } else {
                        value
                    }
                }),
                on_done: Box::new(move |value| {
                    log::info!("New value: {}", value);
                    s.get_component("config", |component: &ConfigComponent| {
                        component.set_bpm(value as u8);
                    });
                })
            }, bpm as i32
        ));

        data.add_element(UIButton::new(
            "Pad Settings".to_string(),
            move || {
                navigator.clone().send(NavigatorMessage::Navigate("pad_settings".to_string())).unwrap();
            }
        ));

        SettingsScreen {
            data
        }
    }
}

impl Screen for SettingsScreen {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }
}