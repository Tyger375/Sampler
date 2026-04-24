use std::any::Any;
use std::fs;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::sequencer::SequencerProject;
use crate::settings::component::SettingsComponent;
use crate::settings::{load_config, load_config_or_default, load_config_or_null, save_config};
use crate::settings_components::SettingsEvent;

pub struct SequencerProjects {
    settings_tx: Sender<SettingsEvent>
}

impl SequencerProjects {
    const FOLDER: &str = "/data/sequencer/projects";

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

    pub fn save_project(&self, project: SequencerProject) {
        let path = format!("{}/{}.toml", Self::FOLDER, project.name);
        save_config::<SequencerProject>(&path, &project);
    }

    pub fn read_projects(&self) -> Vec<String> {
        let result = fs::read_dir(Self::FOLDER).unwrap();
        let files: Vec<String> = result
            .map(|entry| entry.unwrap().file_name().into_string().unwrap().strip_suffix(".toml").unwrap().to_string())
            .collect();
        files
    }

    pub fn get_project(&self, name: &str) -> SequencerProject {
        let path = format!("{}/{}.toml", Self::FOLDER, name);
        load_config::<SequencerProject>(&path)
    }
}

impl SettingsComponent for SequencerProjects {
    fn as_any(&self) -> &dyn Any { self }

    fn direct_read(&self, _args: &Vec<&str>) -> String {
        String::new()
    }
}