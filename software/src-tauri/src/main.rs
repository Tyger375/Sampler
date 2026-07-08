// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use std::io::Write;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, State};

use software_sampler_lib::audio::{create_audio_stream, load_and_decode, AudioData, AudioFile, AudioTrack};
use software_sampler_lib::drumpad::PadConfig;
use software_sampler_lib::midi::{create_midi_task, MIDIEvent};
use software_sampler_lib::handlers::*;
use software_sampler_lib::AppState;

#[derive(Serialize)]
struct MainConfig {
    midi_device: String
}

fn init_configurations() {
    let config = MainConfig {
        midi_device: "MIDI Device".to_string()
    };
    if let Ok(mut file) = fs::File::create("test.json") {
        file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes()).ok();
    }
}

fn main() {
    let samples = load_and_decode(Path::new("/home/edoar/Projects/software-sampler/samples/kick.wav"));
    let samples2 = load_and_decode(Path::new("/home/edoar/RustroverProjects/SoftwareSampler/target/debug/samples/sample2.wav"));
    let samples3 = load_and_decode(Path::new("/home/edoar/RustroverProjects/SoftwareSampler/target/debug/samples/sample3.wav"));

    let audio_data2 = AudioData {
        track: AudioTrack {
            label: "kick".to_string(),
            id: 1,
        },
        file: samples,
    };
    let audio_data3 = AudioData {
        track: AudioTrack {
            label: "snare".to_string(),
            id: 2,
        },
        file: samples2
    };
    let audio_data4 = AudioData {
        track: AudioTrack {
            label: "idk".to_string(),
            id: 3,
        },
        file: samples3
    };

    let mut audio_data: Vec<Arc<AudioData>> = vec![];

    audio_data.push(Arc::new(audio_data2));
    audio_data.push(Arc::new(audio_data3));
    audio_data.push(Arc::new(audio_data4));

    let audio_data = Arc::new(audio_data);

    let pad_configs = Arc::new(Mutex::new([PadConfig::default(); 8]));

    {
        let mut guard = pad_configs.lock().unwrap();
        for (index, config) in guard.iter_mut().enumerate() {
            config.id = (index + 1) as u8;
            config.note = 60 + index as u8;
        }

        guard[0].track_id = 1;
        guard[1].track_id = 2;
        guard[2].track_id = 3;
    }

    let events_rx = create_midi_task();

    let hosts = cpal::available_hosts();
    for host in hosts {
        println!("Available Host: {}", host.name());
    }

    let host = cpal::host_from_id(cpal::HostId::Jack).expect("Jack host");
    println!("Host: {}", host.id());
    let device = host.default_output_device().expect("No output device found");
    println!("Connected to virtual device: {:?}", device.id());

    let audio_cmd_tx = create_audio_stream(device);

    tauri::Builder::default()
        .manage(AppState {
            audio_data: audio_data.clone(),
            pad_configs: pad_configs.clone(),
        })
        .setup(move |app| {
            let app_handle = app.handle().clone();

            let pad_configs = pad_configs.clone();
            {
                let audio_data = audio_data.clone();
                let app_handle = app_handle.clone();

                thread::spawn(move || {
                    loop {
                        if let Ok(event) = events_rx.try_recv() {
                            match event {
                                MIDIEvent::NoteOn(note, velocity) => {
                                    let configs: Vec<PadConfig> = {
                                        let guard = pad_configs.lock().unwrap();
                                        guard.iter()
                                            .filter(|item| item.note == note)
                                            .cloned()
                                            .collect()
                                    };
                                    //let mut guard = playback_states.lock().unwrap();
                                    for config in configs.iter() {
                                        app_handle.emit("pad-event", (config.id, true)).unwrap();

                                        if let Some(matched_audio) = audio_data.iter()
                                            .find(|item| item.track.id == config.track_id) {
                                            let _ = audio_cmd_tx.send(matched_audio.clone());
                                        }
                                    }
                                }
                                MIDIEvent::NoteOff(note) => {
                                    let configs: Vec<PadConfig> = {
                                        let guard = pad_configs.lock().unwrap();
                                        guard.iter()
                                            .filter_map(|item| {
                                                if item.note == note {
                                                    Some(item.clone())
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect()
                                    };
                                    //let mut guard = playback_states.lock().unwrap();
                                    for config in configs.iter() {
                                        app_handle.emit("pad-event", (config.id, false)).unwrap();
                                    }
                                }
                            }
                        }
                    }
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_pad_configs, set_pad_configs,
            get_tracks,
            get_audio_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
