use std::sync::Arc;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use crate::graphics::event::GraphicsEvent;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::leds_manager::LedsManager;
use crate::navigator::{Navigator, NavigatorMessage};
use crate::sequencer::Sequencer;
use crate::utils::{int_to_note, pad_to_seq, seq_to_pad, CustomGraphicsEvent};

pub struct SequencerTracksScreen {
    data: ScreenData,
    navigator: Navigator,
    sequencer: Arc<Sequencer>,
    leds_manager: Arc<LedsManager>,
    track: Arc<AtomicI32>,
    page: Arc<AtomicU32>
}

impl SequencerTracksScreen {
    pub fn factory(
        navigator: Navigator,
        sequencer: Arc<Sequencer>,
        leds_manager: Arc<LedsManager>
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> {
        move |_| Box::new(Self::new(navigator.clone(), sequencer.clone(), leds_manager.clone()))
    }

    pub fn new(
        navigator: Navigator,
        sequencer: Arc<Sequencer>,
        leds_manager: Arc<LedsManager>
    ) -> Self {
        let mut screen = Self {
            data: ScreenData::new(),
            navigator,
            sequencer,
            leds_manager,
            track: Arc::new(AtomicI32::new(-1)),
            page: Arc::new(AtomicU32::new(0)),
        };
        screen.show_tracks();
        screen
    }

    fn show_tracks(&mut self) {
        self.data.clear();

        self.data.add_text("Tracks".to_string());

        let tracks: Vec<String> = self.sequencer.get_project(|project| {
            project.tracks
                .iter()
                .map(|item| int_to_note(item.note as i32))
                .collect()
        });

        let sequencer = self.sequencer.clone();
        let navigator = self.navigator.clone();
        self.data.add_element(UIButton::new(
            "Add".to_string(),
            move || {
                sequencer.add_track();
                navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh)).ok();
            }
        ));

        for (index, track) in tracks.iter().enumerate() {
            let navigator = self.navigator.clone();
            let selected_track = self.track.clone();
            self.data.add_element(UIButton::new(
                format!("{} ({})", index, track),
                move || {
                    selected_track.store(index as i32, Ordering::Relaxed);
                    navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh)).ok();
                }
            ));
        }
    }

    fn show_track(&mut self) {
        self.data.clear();

        let index = self.track.load(Ordering::Relaxed);
        self.data.add_text(format!("Track: {}", index));
    }
}

impl Screen for SequencerTracksScreen {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }

    fn on_back(&mut self) -> bool {
        if self.track.load(Ordering::Relaxed) >= 0 {
            self.track.store(-1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    fn on_custom_event(&mut self, event: u32) -> bool {
        let event = CustomGraphicsEvent::from(event);

        if !event.is_long_click() && !event.is_shortcut() {
            let track = self.track.load(Ordering::Relaxed);
            if track < 0 {
                return false;
            }

            let channel = event.get_channel();
            let pad_to_seq = pad_to_seq(channel);

            let sequencer = self.sequencer.clone();
            let result = sequencer.edit_track(track as usize, |track| {
                if let Some(pos) = track.triggers.iter().position(|&t| t == pad_to_seq) {
                    track.triggers.remove(pos);
                } else {
                    track.triggers.push(pad_to_seq);
                }

                track.triggers.clone()
            }).unwrap();

            log::info!("Pad pressed for sequencer: {} {}", channel, pad_to_seq);
            let mut status = 0u8;
            for &trigger in result.iter() {
                status |= 1 << seq_to_pad(trigger);
            }
            self.leds_manager.set_sequencer(status, true);
        }

        false
    }

    fn refresh(&mut self) {
        self.data.row_offset = 0;
        if self.track.load(Ordering::Relaxed) < 0 {
            self.show_tracks();
        } else {
            self.show_track()
        }
    }
}