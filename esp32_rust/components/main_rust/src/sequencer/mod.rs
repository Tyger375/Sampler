use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use serde::{Deserialize, Serialize};
use crate::midi::MIDI;

#[repr(u8)]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum SequencerResolution {
    Quarter     = 1,
    HalfBeat    = 2,
    Beat        = 4,
    Loop        = 16,
}

impl SequencerResolution {
    pub fn as_index(&self) -> i32 {
        match self {
            SequencerResolution::Quarter => 0,
            SequencerResolution::HalfBeat => 1,
            SequencerResolution::Beat => 2,
            SequencerResolution::Loop => 3
        }
    }

    pub fn from_index(index: i32) -> Self {
        match index {
            0 => SequencerResolution::Quarter,
            1 => SequencerResolution::HalfBeat,
            2 => SequencerResolution::Beat,
            3 => SequencerResolution::Loop,
            _ => panic!("Couldn't convert {} into SequencerResolution", index)
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SequencerTrack {
    pub loops: u8,
    pub resolution: SequencerResolution,
    pub note: u8,
    pub triggers: Vec<u8>,
}

impl Default for SequencerTrack {
    fn default() -> Self {
        Self {
            loops: 1,
            resolution: SequencerResolution::HalfBeat,
            note: 60,
            triggers: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SequencerProject {
    pub name: String,
    pub loops: u8,
    pub tracks: Vec<SequencerTrack>
}

pub struct Sequencer {
    project: Arc<Mutex<Option<SequencerProject>>>,
    trigger_states: Arc<Mutex<Vec<bool>>>,
    enable: AtomicBool,
    current_loop: AtomicU8,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            project: Arc::new(Mutex::new(None)),
            trigger_states: Arc::new(Mutex::new(vec![])),
            enable: AtomicBool::new(false),
            current_loop: AtomicU8::new(0)
        }
    }

    pub fn enabled(&self) -> bool {
        self.enable.load(Ordering::Relaxed)
    }

    pub fn set_enable(&self, value: bool) {
        self.enable.store(value, Ordering::Relaxed);
    }

    pub fn new_project(&self, name: String) {
        let project = SequencerProject {
            name,
            loops: 1,
            tracks: vec![]
        };
        {
            let mut guard = self.project.lock().unwrap();
            *guard = Some(project);
        }

        {
            let mut guard = self.trigger_states.lock().unwrap();
            guard.clear();
        }
    }

    pub fn load_project(&self, project: SequencerProject) {
        let tracks_num = project.tracks.len();
        {
            let mut guard = self.project.lock().unwrap();
            *guard = Some(project);
        }
        {
            let mut guard = self.trigger_states.lock().unwrap();
            guard.resize(tracks_num, false);
        }
    }

    pub fn get_project<F, T>(&self, lambda: F) -> T
    where F: FnOnce(&SequencerProject) -> T {
        let guard = self.project.lock().unwrap();
        lambda(guard.as_ref().expect("Project is None"))
    }

    pub fn get_project_name(&self) -> String {
        let guard = self.project.lock().unwrap();
        let project = guard.as_ref().unwrap();
        project.name.clone()
    }

    pub fn get_project_name_or_null(&self) -> Option<String> {
        let guard = self.project.lock().unwrap();
        if let Some(project) = guard.as_ref() {
            return Some(project.name.clone());
        }
        None
    }

    pub fn set_project_loops(&self, loops: u8) {
        let mut guard = self.project.lock().unwrap();
        let project = guard.as_mut().unwrap();
        project.loops = loops;

        // Adjusting track loops
        for track in project.tracks.iter_mut() {
            track.loops = 1;
        }
    }

    pub fn add_track(&self) {
        let tracks_num = {
            let mut guard = self.project.lock().unwrap();
            if guard.is_none() {
                return;
            }
            let guard = guard.as_mut().unwrap();
            guard.tracks.push(SequencerTrack::default());
            guard.tracks.len()
        };
        {
            let mut guard = self.trigger_states.lock().unwrap();
            guard.resize(tracks_num, false);
        }
    }

    pub fn remove_track(&self, index: usize) {
        {
            let mut guard = self.project.lock().unwrap();
            if guard.is_none() {
                return;
            }
            let guard = guard.as_mut().unwrap();
            guard.tracks.remove(index);
        }
        {
            let mut guard = self.trigger_states.lock().unwrap();
            guard.remove(index);
        }
    }

    pub fn edit_track<F, T>(&self, index: usize, lambda: F) -> Option<T>
    where F: FnOnce(&mut SequencerTrack) -> T {
        let mut guard = self.project.lock().unwrap();
        let proj = guard.as_mut().unwrap();
        if let Some(track) = proj.tracks.get_mut(index) {
            Some(lambda(track))
        } else {
            None
        }
    }

    pub fn step_trigger_on(&self, step: u8) {
        let project = {
            let guard = self.project.lock().unwrap();
            guard.clone()
        };
        if project.is_none() {
            return;
        }
        let mut guard = self.trigger_states.lock().unwrap();

        let project = project.unwrap();
        let current_loop = self.current_loop.load(Ordering::Relaxed);
        for (track, state) in project.tracks.iter().zip(guard.iter_mut()) {
            let track_step = step + (16 * (current_loop % track.loops));
            Self::handle_track_on(track, track_step, state);
        }
    }
    pub fn step_trigger_off(&self, step: u8) {
        let project = {
            let guard = self.project.lock().unwrap();
            guard.clone()
        };
        if project.is_none() {
            return;
        }
        let mut guard = self.trigger_states.lock().unwrap();

        let project = project.unwrap();
        let current_loop = self.current_loop.load(Ordering::Relaxed);
        for (track, state) in project.tracks.iter().zip(guard.iter_mut()) {
            let track_step = step + (16 * (current_loop % track.loops));
            Self::handle_track_off(track, track_step, state);
        }

        if step == 15 {
            self.current_loop.store((current_loop + 1) % project.loops, Ordering::Relaxed);
        }
    }

    fn handle_track_on(track: &SequencerTrack, step: u8, state_trigger: &mut bool) {
        if *state_trigger {
            return;
        }

        for trigger in track.triggers.iter() {
            let real_trigger = trigger * (track.resolution as u8);
            if real_trigger == step {
                *state_trigger = true;
                MIDI::send(&[0x09, 0x90, track.note, 127]);
            }
        }
    }

    fn handle_track_off(track: &SequencerTrack, step: u8, state_trigger: &mut bool) {
        if !(*state_trigger) {
            return;
        }

        for trigger in track.triggers.iter() {
            let real_trigger = ((trigger + 1) * (track.resolution as u8)) - 1;
            if real_trigger == step {
                *state_trigger = false;
                MIDI::send(&[0x08, 0x80, track.note, 0]);
            }
        }
    }
}
