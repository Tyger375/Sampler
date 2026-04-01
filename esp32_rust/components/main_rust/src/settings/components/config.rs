use std::any::Any;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::Sender;
use serde::{Deserialize, Serialize};
use crate::settings::component::SettingsComponent;
use crate::settings::{load_config, save_config};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub bpm: u8
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            bpm: 140
        }
    }
}

pub struct ConfigComponent {
    settings_tx: Sender<String>,
    data: Mutex<ConfigData>,
    bpm: Arc<AtomicU8>
}

impl SettingsComponent for ConfigComponent {
    fn as_any(&self) -> &dyn Any { self }
}

impl ConfigComponent {
    pub fn new(
        settings_tx: Sender<String>
    ) -> Self {
        let component = ConfigComponent {
            data: Mutex::new(ConfigData::default()),
            settings_tx,
            bpm: Arc::new(AtomicU8::new(0))
        };
        component.on_load();
        component
    }

    pub fn unwrap_data(&self) {
        let guard = self.data.lock().unwrap();

        self.bpm.store(guard.bpm, Ordering::Relaxed);
    }

    pub fn on_load(&self) {
        let config = load_config::<ConfigData>("/data/config.toml");

        {
            let mut guard = self.data.lock().unwrap();
            *guard = config;
        }

        self.unwrap_data();
    }

    pub fn save(&self) {
        let data = {
            let guard = self.data.lock().unwrap();
            guard.clone()
        };
        save_config::<ConfigData>("/data/config.toml", &data);
        log::info!(target: "ConfigComponent", "Saved!");

        self.unwrap_data();
    }

    pub fn bpm(&self) -> u8 {
        self.bpm.load(Ordering::Relaxed)
        /*let guard = self.data.lock().unwrap();
        guard.bpm*/
    }

    pub fn set_bpm(&self, bpm: u8) {
        {
            let mut guard = self.data.lock().unwrap();
            guard.bpm = bpm;
        }
        self.settings_tx.send("config_bpm".to_string()).unwrap();
    }
}