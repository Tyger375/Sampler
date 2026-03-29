use crate::graphics::ui::element::UIElement;

pub struct UIText {
    text: String
}

impl UIText {
    pub fn new(value: String) -> UIText {
        UIText { text: value }
    }
}

impl UIElement for UIText {
    fn render(&self, selected: bool) -> String {
        let prefix = if selected {
            ">"
        } else {
            ""
        };
        format!("{}{}", prefix, self.text)
    }
}
