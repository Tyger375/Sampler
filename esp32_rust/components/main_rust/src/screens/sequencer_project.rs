use std::sync::Arc;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::text::UIText;
use crate::navigator::{Navigator, NavigatorMessage};
use crate::sequencer::Sequencer;

pub struct SequencerProjectScreen {
    data: ScreenData
}

impl SequencerProjectScreen {
    pub fn factory(
        navigator: Navigator,
        sequencer: Arc<Sequencer>
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> {
        move |_| {
            Box::new(Self::new(navigator.clone(), sequencer.clone()))
        }
    }

    pub fn new(
        navigator: Navigator,
        sequencer: Arc<Sequencer>
    ) -> Self {
        let mut data = ScreenData::new();

        let (name, loops, tracks) = sequencer.get_project(|project| {
            (project.name.clone(), project.loops, project.tracks.len())
        });
        data.add_text(format!("{} {}", name, loops));
        data.add_element(UIButton::new(
            format!("Tracks: {}", tracks),
            move || {
                navigator.send(NavigatorMessage::Navigate("sequencer_tracks".to_string())).ok();
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