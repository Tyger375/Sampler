use std::any::Any;
use std::sync::mpsc::Sender;
use crate::settings::component::SettingsComponent;
use crate::settings_components::SettingsEvent;
use crate::utils::CustomGraphicsEvent;

pub struct ShortcutsComponent {
    settings_tx: Sender<SettingsEvent>
}

pub enum Shortcut {
    NavigateScreen(String)
}

impl SettingsComponent for ShortcutsComponent {
    fn as_any(&self) -> &dyn Any { self }

    fn direct_read(&self, _args: &Vec<&str>) -> String {
        todo!()
    }
}

impl ShortcutsComponent {
    pub fn new(
        settings_tx: Sender<SettingsEvent>
    ) -> Self {
        let component = Self {
            settings_tx
        };
        component.on_load();
        component
    }

    fn on_load(&self) {

    }

    pub fn from_cevent(&self, event: CustomGraphicsEvent) -> Option<Shortcut> {
        if !event.is_shortcut() {
            return None;
        }

        if !event.is_long_click() && event.get_channel() == 0 {
            return Some(Shortcut::NavigateScreen("settings".to_string()));
        }

        return None;
    }
}