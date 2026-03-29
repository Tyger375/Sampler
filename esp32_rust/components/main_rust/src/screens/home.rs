use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::text::UIText;

pub struct HomeScreen {
    data: ScreenData
}

impl HomeScreen {
    pub(crate) fn new() -> Self {
        let mut data = ScreenData::new();

        let title = UIText::new("Hello World!".to_string());
        data.add_element(title);

        let text = UIText::new("Test".to_string());
        data.add_element(text);

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
