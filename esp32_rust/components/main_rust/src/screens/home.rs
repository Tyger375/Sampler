use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::row::UIRow;
use crate::graphics::ui::text::UIText;
use crate::navigator::{Navigator, NavigatorMessage};

pub struct HomeScreen {
    data: ScreenData
}

impl HomeScreen {
    pub fn factory(
        navigator: Navigator
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> + 'static {
        move |_| Box::new(HomeScreen::new(navigator.clone()))
    }

    pub fn new(navigator: Navigator) -> Self {
        let mut data = ScreenData::new();

        let title = UIText::new("Home".to_string());
        data.add_element(title);

        let mut row = UIRow::new();

        let nav = navigator.clone();
        let settings_btn = UIButton::new("Settings".to_string(), move || {
            log::info!("Navigating to settings");
            nav.clone().send(NavigatorMessage::Navigate("settings".to_string())).ok();
        });
        row.add_element(settings_btn);

        let nav = navigator.clone();
        let sequencer_btn = UIButton::new("Sequencer".to_string(), move || {
            nav.clone().send(NavigatorMessage::Navigate("sequencer".to_string())).ok();
        });
        row.add_element(sequencer_btn);

        data.add_element(row);

        HomeScreen {
            data
        }
    }
}

impl Screen for HomeScreen {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }
}
