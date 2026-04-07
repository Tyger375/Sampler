use std::sync::Arc;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::text::UIText;
use crate::navigator::{Navigator, NavigatorMessage};
use crate::sequencer::Sequencer;

pub struct SequencerScreen {
    data: ScreenData
}

impl SequencerScreen {
    pub fn factory(
        navigator: Navigator,
        sequencer: Arc<Sequencer>
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> {
        move |_| Box::new(SequencerScreen::new(navigator.clone(), sequencer.clone()))
    }

    pub fn new(
        navigator: Navigator,
        sequencer: Arc<Sequencer>
    ) -> Self {
        let mut data = ScreenData::new();

        data.add_element(UIText::new("Sequencer".to_string()));

        let seq = sequencer.clone();
        data.add_element(UIButton::new("Enable".to_string(), || {
            //seq.clone()
        }));

        let nav = navigator.clone();
        data.add_element(UIButton::new("New project".to_string(), move || {
            seq.clone().new_project("Project".to_string(), 4);
            nav.send(NavigatorMessage::Navigate("sequencer_project".to_string())).ok();
        }));

        Self {
            data
        }
    }
}

impl Screen for SequencerScreen {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }
}