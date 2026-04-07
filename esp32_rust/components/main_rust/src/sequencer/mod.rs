use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use crate::midi::MIDI;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SequencerResolution {
    Quarter     = 1,
    HalfBeat    = 2,
    Beat        = 4,
    Loop        = 16,
}

#[derive(Clone)]
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
            resolution: SequencerResolution::Quarter,
            note: 60,
            triggers: vec![]
        }
    }
}

#[derive(Clone)]
pub struct SequencerProject {
    pub name: String,
    pub loops: u8,
    pub tracks: Vec<SequencerTrack>
}

pub struct Sequencer {
    project: Arc<Mutex<Option<SequencerProject>>>,
    trigger_states: Arc<Mutex<Vec<bool>>>,
    enable: bool,
    current_loop: AtomicU8,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            project: Arc::new(Mutex::new(None)),
            trigger_states: Arc::new(Mutex::new(vec![])),
            enable: false,
            current_loop: AtomicU8::new(0)
        }
    }

    pub fn new_project(&self, name: String, loops: u8) {
        let project = SequencerProject {
            name,
            loops,
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

    pub fn get_project<T>(&self, lambda: fn(&SequencerProject) -> T) -> T {
        let guard = self.project.lock().unwrap();
        lambda(guard.as_ref().expect("Project is None"))
    }

    pub fn get_project_name(&self) -> String {
        let guard = self.project.lock().unwrap();
        let project = guard.as_ref().unwrap();
        project.name.clone()
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

        if step == 15 {
            self.current_loop.store((current_loop + 1) % project.loops, Ordering::Relaxed);
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
