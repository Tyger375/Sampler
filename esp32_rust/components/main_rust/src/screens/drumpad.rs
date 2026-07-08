use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::navigator::{Navigator, NavigatorMessage};

pub struct DrumPadScreen {
    data: ScreenData
}

impl DrumPadScreen {
    pub fn factory(
        navigator: Navigator
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> + 'static {
        move |_| { Box::new(Self::new(navigator.clone())) }
    }

    pub fn new(
        navigator: Navigator
    ) -> Self {
        let mut data = ScreenData::new();

        data.add_text("DrumPad".to_string());

        data.add_element(UIButton::new(
            "pads".to_string(),
            move || {
                navigator.clone()
                    .send(NavigatorMessage::Navigate("pad_settings".to_string()))
                    .ok();
            }
        ));

        Self {
            data
        }
    }
}

impl Screen for DrumPadScreen {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }
}