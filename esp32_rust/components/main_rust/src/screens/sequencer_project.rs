use std::sync::Arc;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::intinput::{IntInputConfig, UIIntInput};
use crate::navigator::{Navigator, NavigatorMessage};
use crate::sequencer::Sequencer;
use crate::settings::manager::SettingsManager;
use crate::settings_components::sequencer_projects::SequencerProjects;
use crate::settings_components::SettingsEvent;

pub struct SequencerProjectScreen {
    data: ScreenData
}

impl SequencerProjectScreen {
    pub fn factory(
        navigator: Navigator,
        sequencer: Arc<Sequencer>,
        settings_manager: Arc<SettingsManager<SettingsEvent>>
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> {
        move |_| {
            Box::new(Self::new(navigator.clone(), sequencer.clone(), settings_manager.clone()))
        }
    }

    pub fn new(
        navigator: Navigator,
        sequencer: Arc<Sequencer>,
        settings_manager: Arc<SettingsManager<SettingsEvent>>
    ) -> Self {
        let mut data = ScreenData::new();

        let (name, loops, tracks) = sequencer.get_project(|project| {
            (project.name.clone(), project.loops, project.tracks.len())
        });
        data.add_text(name);

        let seq = sequencer.clone();
        data.add_element(UIIntInput::new(
            IntInputConfig::new(
                "Loops",
                |value| {
                    value.to_string()
                },
                |value, _| {
                    if value <= 0 {
                        1
                    } else if value > 16 {
                        16
                    } else {
                        value
                    }
                },
                move |value| {
                    seq.set_project_loops(value as u8);
                }
            ), loops as i32
        ));
        data.add_element(UIButton::new(
            format!("Tracks: {}", tracks),
            move || {
                navigator.send(NavigatorMessage::Navigate("sequencer_tracks".to_string())).ok();
            }
        ));

        let settings = settings_manager.clone();
        let seq = sequencer.clone();
        data.add_element(UIButton::new(
            "Save".to_string(),
            move || {
                let project = seq.get_project(|project| {
                    project.clone()
                });
                settings.get_component("sequencer_projects", |component: &SequencerProjects| {
                    component.save_project(project);
                    log::info!("Saved!");
                });
            }
        ));

        Self {
            data
        }
    }
}

impl Screen for SequencerProjectScreen {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }
}