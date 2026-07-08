use tauri::State;
use crate::AppState;
use crate::audio::{AudioFile, AudioTrack};
use crate::drumpad::PadConfig;

#[tauri::command]
pub fn get_pad_configs(state: State<'_, AppState>) -> Vec<PadConfig> {
    let guard = state.pad_configs.lock().unwrap();
    guard.to_vec()
}

#[tauri::command]
pub fn set_pad_configs(state: State<'_, AppState>, pad_configs: Vec<PadConfig>) {
    let mut guard = state.pad_configs.lock().unwrap();
    for (config, new_config) in guard.iter_mut().zip(pad_configs) {
        *config = new_config;
    }
}

#[tauri::command]
pub fn get_tracks(state: State<'_, AppState>) -> Vec<AudioTrack> {
    state.audio_data.iter()
        .map(|item| AudioTrack {
            label: item.track.label.clone(),
            id: item.track.id
        })
        .collect()
}

#[tauri::command]
pub fn get_audio_file(state: State<'_, AppState>, track_id: usize) -> Option<AudioFile> {
    state.audio_data.iter()
        .find(|item| item.track.id == track_id)
        .map(|item| item.file.clone())
}