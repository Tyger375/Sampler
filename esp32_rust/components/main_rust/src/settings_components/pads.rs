use std::any::Any;
use std::{array, fs};
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::pads::PadPressType;
use crate::settings::component::SettingsComponent;
use crate::settings::{load_config_or_default, save_config};
use crate::settings_components::SettingsEvent;
use crate::utils::{MAX_MIDI_CHANNELS, MAX_MIDI_NOTE};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PadConfig {
    pub note: u8,
    pub channel: u8,
    pub press_type: PadPressType,
    pub threshold: u16
}

impl PadConfig {
    fn new_list<const N: usize>() -> [Self; N] {
        array::from_fn(|i| {
            PadConfig {
                note: 60 + i as u8,
                channel: 0,
                press_type: PadPressType::OneShot,
                threshold: 50
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PadsData {
    pads: [PadConfig; 8]
}

impl Default for PadsData {
    fn default() -> Self {
        PadsData {
            pads: PadConfig::new_list()
        }
    }
}

pub struct PadsComponent {
    settings_tx: Sender<SettingsEvent>,
    data: Mutex<PadsData>,
    pads: Mutex<[PadConfig; 8]>
}

impl SettingsComponent for PadsComponent {
    fn as_any(&self) -> &dyn Any { self }

    fn direct_read(&self, _args: &Vec<&str>) -> String {
        fs::read_to_string(Self::FILENAME).unwrap_or(String::new())
    }
}

impl PadsComponent {
    const FILENAME: &str = "/data/pads.toml";

    pub fn new(settings_tx: Sender<SettingsEvent>) -> Self {
        let component = PadsComponent {
            settings_tx,
            data: Mutex::new(PadsData::default()),
            pads: Mutex::new(PadConfig::new_list())
        };
        component.on_load();
        component
    }

    fn unwrap_data(&self) {
        let data_guard = self.data.lock().unwrap();

        {
            let mut pads_guard = self.pads.lock().unwrap();
            *pads_guard = data_guard.pads.clone();
        }
    }

    pub fn on_load(&self) {
        let config = load_config_or_default::<PadsData>(Self::FILENAME);

        {
            let mut guard = self.data.lock().unwrap();
            *guard = config;
        }

        self.unwrap_data();
    }

    pub fn save(&self) {
        let data = {
            let mut data_guard = self.data.lock().unwrap();

            {
                let pads_guard = self.pads.lock().unwrap();
                (*data_guard).pads = pads_guard.clone();
            }
            (*data_guard).clone()
        };

        save_config::<PadsData>(Self::FILENAME, &data);
        log::info!(target: "PadsComponent", "Saved!");

        self.unwrap_data();
    }

    pub fn commit(&self) {
        self.settings_tx.send(SettingsEvent::PadConfig).unwrap();
    }

    pub fn get_configs(&self) -> [PadConfig; 8] {
        let guard = self.pads.lock().unwrap();
        (*guard).clone()
    }

    pub fn get_data_config(&self, index: u8) -> PadConfig {
        let guard = self.pads.lock().unwrap();
        guard[index as usize].clone()
    }

    pub fn set_pad_note(&self, index: u8, note: u8) {
        if note > MAX_MIDI_NOTE {
            return;
        }

        let mut guard = self.pads.lock().unwrap();
        guard[index as usize].note = note;
    }

    pub fn set_pad_channel(&self, index: u8, channel: u8) {
        if channel > MAX_MIDI_CHANNELS {
            return;
        }

        let mut guard = self.pads.lock().unwrap();
        guard[index as usize].channel = channel;
    }

    pub fn set_pad_threshold(&self, index: u8, threshold: u16) {
        let mut guard = self.pads.lock().unwrap();
        guard[index as usize].threshold = threshold;
    }
}
