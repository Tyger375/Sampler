use std::sync::{Arc, Mutex};
use crate::audio::AudioData;
use crate::drumpad::PadConfig;

pub mod audio;
pub mod drumpad;
pub mod midi;
pub mod handlers;

pub struct AppState {
    pub audio_data: Arc<Vec<Arc<AudioData>>>,
    pub pad_configs: Arc<Mutex<[PadConfig; 8]>>
}
