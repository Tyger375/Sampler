use std::fs::File;
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::park;
use cpal::StreamConfig;
use cpal::traits::DeviceTrait;
use serde::{Deserialize, Serialize};
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

#[derive(Clone, Serialize)]
pub struct AudioFile {
    pub path: String,
    pub data: Vec<f32>,
    pub channels: u32,
    pub sample_rate: u32
}

#[derive(Serialize, Deserialize)]
pub struct AudioTrack {
    pub label: String,
    pub id: usize,
}

pub struct AudioData {
    pub track: AudioTrack,
    pub file: AudioFile,
}

pub fn load_and_decode(path: &Path) -> AudioFile {
    let src = File::open(path).expect("Failed to open file");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let hint = Hint::new();
    let mut format = symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
        .expect("unsupported format");

    // Find the first audio track with a known (decodable) codec.
    let track = format
        .default_track(TrackType::Audio)
        .expect("no audio track");

    // Create decoder for the track
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(
            track.codec_params.as_ref().expect("codec parameters missing")
                .audio().unwrap(),
            &AudioDecoderOptions::default()
        )
        .expect("unsupported codec");

    let track_id = track.id;
    let mut pcm_data = Vec::new();
    let mut sample_buf = Vec::new();

    let params = track.codec_params.as_ref().unwrap().audio().unwrap();

    let channels = params.channels.as_ref().unwrap().count() as u32;
    let sample_rate = params.sample_rate.unwrap();
    println!("{:?} {:?}", params.sample_rate, params.channels);

    loop {
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => break,
            Err(Error::IoError(_)) => break, // End of file
            Err(e) => panic!("error reading packet: {}", e),
        };

        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

            if let Some(rev) = format.metadata().current() {
                // Consume the new metadata at the head of the metadata queue.
                println!("{:?}", rev);
            }
        }

        if packet.track_id != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                sample_buf.clear();
                decoded.copy_to_vec_interleaved(&mut sample_buf);
                pcm_data.extend_from_slice(&sample_buf);
            },
            Err(Error::DecodeError(_)) => continue,
            Err(_) => break,
        }
    }

    AudioFile {
        path: path.to_string_lossy().to_string(),
        data: pcm_data,
        channels,
        sample_rate,
    }
}

struct PlaybackState {
    current_frame: usize,
    audio_data: Arc<AudioData>
}

pub fn create_audio_stream(device: cpal::Device) -> mpsc::Sender<Arc<AudioData>> {
    let config: StreamConfig = device.default_output_config()
        .expect("Failed to get default config")
        .into();
    println!("{}", config.sample_rate);

    let (audio_cmd_tx, audio_cmd_rx) = mpsc::channel::<Arc<AudioData>>();

    thread::spawn(move || {
        let mut active_voices: Vec<PlaybackState> = Vec::with_capacity(32);
        
        let _stream = device.build_output_stream(&config, move |data: &mut [f32], _| {
            /*let mut states: Vec<_> = {
                let guard = playback_states.lock().unwrap();
                guard.iter()
                    .map(|s| (s.current_frame, s.audio_data.clone()))
                    .collect()
            };*/
            while let Ok(new_audio) = audio_cmd_rx.try_recv() {
                active_voices.push(PlaybackState {
                    audio_data: new_audio,
                    current_frame: 0
                });
            }

            for sample in data.iter_mut() {
                *sample = 0.0;
            }

            let mut remove_indices: Vec<usize> = Vec::new();

            for frame in data.chunks_mut(2) {
                for (index, voice)in active_voices.iter_mut().enumerate() {
                    let cursor = voice.current_frame;
                    let file_data = &voice.audio_data.file.data;

                    if cursor < file_data.len() {
                        match voice.audio_data.file.channels {
                            1 => {
                                let s = file_data[cursor];
                                frame[0] += s;
                                frame[1] += s;
                                voice.current_frame += 1;
                            }
                            2 => {
                                let s1 = file_data[cursor];
                                let s2 = file_data[cursor + 1];
                                frame[0] += s1;
                                frame[1] += s2;
                                voice.current_frame += 2;
                            },
                            _ => ()
                        };
                    } else if !remove_indices.contains(&index) {
                        remove_indices.push(index);
                    }
                }
            }

            if !remove_indices.is_empty() {
                // Sort backwards so index shifting doesn't break removal
                remove_indices.sort_by(|a, b| b.cmp(a));
                for idx in remove_indices {
                    active_voices.swap_remove(idx);
                }
            }
        }, |_| {}, None).unwrap();

        park();
    });

    audio_cmd_tx
}