use std::any::Any;
use std::fs;
use std::sync::mpsc::Sender;
use crate::settings::component::SettingsComponent;
use crate::settings_components::SettingsEvent;

pub struct SequencerSettings {
    settings_tx: Sender<SettingsEvent>
}

impl SequencerSettings {
    const FOLDER: &str = "/data/sequencer";
    pub fn new(settings_tx: Sender<SettingsEvent>) -> Self {
        let component = Self {
            settings_tx
        };
        component.on_load();
        component
    }

    fn on_load(&self) {
        if let Err(_) = fs::create_dir(Self::FOLDER) {
            log::warn!("Failed to create folder, it probably already exists");
        }
    }
}

impl SettingsComponent for SequencerSettings {
    fn as_any(&self) -> &dyn Any { self }
    fn direct_read(&self, _args: &Vec<&str>) -> String {
        String::new()
    }
}
