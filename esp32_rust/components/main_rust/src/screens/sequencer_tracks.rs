use std::cmp::{max, min};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use crate::graphics::event::GraphicsEvent;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::element::{UIElement, UIElementState};
use crate::graphics::ui::intinput::{IntInputConfig, UIIntInput};
use crate::graphics::ui::page_selector::UIPageSelector;
use crate::leds_manager::LedsManager;
use crate::navigator::{Navigator, NavigatorMessage};
use crate::sequencer::{Sequencer, SequencerResolution};
use crate::utils::{int_to_note, pad_to_seq, seq_to_pad, CustomGraphicsEvent, MAX_MIDI_NOTE};

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
        self.data.row_offset = 0;
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
                navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh("add_track".to_string()))).ok();
            }
        ));

        for (index, track) in tracks.iter().enumerate() {
            let navigator = self.navigator.clone();
            let selected_track = self.track.clone();
            self.data.add_element(UIButton::new(
                format!("{} ({})", index, track),
                move || {
                    selected_track.store(index as i32, Ordering::Relaxed);
                    navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh("selected_track".to_string()))).ok();
                }
            ));
        }
    }

    fn show_track(&mut self) {
        self.data.row_offset = 0;
        self.data.clear();

        let index = self.track.load(Ordering::Relaxed);
        let (resolution, track_loops, project_loops, track_note) = self.sequencer.get_project(|project| {
            if let Some(track) = project.tracks.get(index as usize) {
                Some((track.resolution, track.loops, project.loops, track.note))
            } else {
                None
            }
        }).unwrap();

        self.data.add_text(format!("Track: {}", index));

        let sequencer = self.sequencer.clone();
        let selected_track = self.track.clone();
        let navigator = self.navigator.clone();
        self.data.add_element(UIButton::new(
            "Remove".to_string(),
            move || {
                sequencer.remove_track(index as usize);
                selected_track.store(-1, Ordering::Relaxed);
                navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh("selected_track".to_string()))).ok();
            }
        ));

        let pages_per_loop = 1 + ((resolution as u8) % 2);
        let pages = pages_per_loop * track_loops;

        let page1 = self.page.clone();
        let page2 = self.page.clone();
        let navigator = self.navigator.clone();
        self.data.add_element(UIPageSelector::new(
            move |is_right| {
                let current_page = page1.load(Ordering::Relaxed);
                let value = if is_right {
                    1
                } else {
                    -1
                };

                let new_page = min(max(current_page as i32 + value, 0), (pages as i32) - 1) as u32;
                page1.store(new_page, Ordering::Relaxed);

                navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh("page".to_string()))).ok();
            },
            move || {
                format!("{}/{}", page2.load(Ordering::Relaxed) + 1, pages)
            }
        ));

        let sequencer = self.sequencer.clone();
        self.data.add_element(UIIntInput::new(
            IntInputConfig {
                text: "Note".to_string(),
                format_value: Box::new(|value| {
                    int_to_note(value)
                }),
                on_change: Box::new(|value, _| {
                    if value < 0 {
                        0
                    } else if value > MAX_MIDI_NOTE as i32 {
                        MAX_MIDI_NOTE as i32
                    } else {
                        value
                    }
                }),
                on_done: Box::new(move |value| {
                    sequencer.edit_track(index as usize, |track| {
                        track.note = value as u8;
                    });
                })
            },
            track_note as i32
        ));

        let sequencer = self.sequencer.clone();
        let navigator = self.navigator.clone();
        self.data.add_element(UIIntInput::new(
            IntInputConfig::new(
                "Res",
                |value| {
                    match value {
                        0 => "Quarter",
                        1 => "HalfBeat",
                        2 => "Beat",
                        3 => "Loop",
                        _ => "-"
                    }.to_string()
                },
                |value, _| {
                    if value < 0 {
                        0
                    } else if value > 3 {
                        3
                    } else {
                        value
                    }
                },
                move |value| {
                    sequencer.edit_track(index as usize, |track| {
                        track.resolution = SequencerResolution::from_index(value);
                    });
                    navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh("refresh_track".to_string()))).ok();
                }
            ), resolution.as_index()
        ));

        let sequencer = self.sequencer.clone();
        let navigator = self.navigator.clone();
        self.data.add_element(UIIntInput::new(
            IntInputConfig::new(
                "Loops",
                |value| {
                    value.to_string()
                },
                move |mut value, old_value| {
                    if value <= 0 {
                        1
                    } else if value > project_loops as i32 {
                        project_loops as i32
                    } else {
                        let changer = value - old_value;
                        while (project_loops as i32 % value) != 0 {
                            value += changer;
                        }
                        value
                    }
                },
                move |value| {
                    sequencer.edit_track(index as usize, |track| {
                        track.loops = value as u8;
                    });
                    navigator.send(NavigatorMessage::GraphicsEvent(GraphicsEvent::Refresh("refresh_track".to_string()))).ok();
                }
            ), track_loops as i32
        ));
    }

    fn update_leds(&self) {
        let track = self.track.load(Ordering::Relaxed);
        if track < 0 {
            self.leds_manager.set_sequencer(0, false);
            return;
        }

        let page = self.page.load(Ordering::Relaxed) as u8;
        let result: Vec<u8> = self.sequencer.get_project(|project| {
            project.tracks[track as usize].triggers
                .iter()
                .filter_map(|&trigger| {
                    if trigger >= (8 * page) && trigger <= 7 + (8 * page) {
                        Some(trigger - 8 * page)
                    } else {
                        None
                    }
                })
                .collect()
        });

        let mut status = 0u8;
        for &trigger in result.iter() {
            status |= 1 << seq_to_pad(trigger);
        }
        self.leds_manager.set_sequencer(status, true);
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
        let data = self.get_data_mut();
        if data.focus {
            data.focus = false;
            return data.elements[data.row_offset].on_event(GraphicsEvent::Back);
        }

        if self.track.load(Ordering::Relaxed) >= 0 {
            self.track.store(-1, Ordering::Relaxed);
            self.refresh("selected_track".to_string());
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

            let page = self.page.load(Ordering::Relaxed) as u8;

            let sequencer = self.sequencer.clone();
            let position = pad_to_seq + 8 * page;
            sequencer.edit_track(track as usize, |track| {
                if let Some(pos) = track.triggers.iter().position(|&t| t == position) {
                    track.triggers.remove(pos);
                } else {
                    track.triggers.push(position);
                }
            });

            self.update_leds();
        }

        false
    }

    fn refresh(&mut self, message: String) {
        match message.as_str() {
            "add_track" => self.show_tracks(),
            "selected_track" => {
                self.page.store(0, Ordering::Relaxed);
                if self.track.load(Ordering::Relaxed) < 0 {
                    self.show_tracks()
                } else {
                    self.show_track()
                }
            },
            "page" => {

            },
            "refresh_track" => {
                self.show_track();
            }
            _ => {}
        }

        self.update_leds();
    }
}