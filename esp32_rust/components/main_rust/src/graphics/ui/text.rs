use crate::graphics::ui::element::{UIElement, UIElementState};

pub struct UIText {
    text: String
}

impl UIText {
    pub fn new(value: String) -> UIText {
        UIText { text: value }
    }
}

impl UIElement for UIText {
    fn render(&self, state: UIElementState) -> String {
        let prefix = match state {
            UIElementState::Selected => ">",
            UIElementState::SelectorPress => "\\",
            UIElementState::None => ""
        };
        format!("{}{}", prefix, self.text)
    }
}
