use std::sync::mpsc::Sender;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::text::UIText;

pub struct HomeScreen {
    data: ScreenData
}

impl HomeScreen {
    pub fn factory(navigator: Sender<String>) -> impl Fn() -> Box<dyn Screen> {
        move || Box::new(HomeScreen::new(navigator.clone()))
    }

    pub fn new(navigator: Sender<String>) -> Self {
        let mut data = ScreenData::new();

        let title = UIText::new("Home".to_string());
        data.add_element(title);

        let nav = navigator.clone();
        let button = UIButton::new("Settings".to_string(), move || {
            log::info!("Navigating to settings");
            nav.send("settings".to_string()).unwrap();
        });
        data.add_element(button);

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
