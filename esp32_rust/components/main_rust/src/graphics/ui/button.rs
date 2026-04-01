use crate::graphics::{event::GraphicsEvent, ui::element::UIElement};

pub struct UIButton {
    text: String,
    on_click: Box<dyn Fn()>
}

impl UIButton {
    pub fn new(text: String, on_click: Box<dyn Fn()>) -> Self {
        UIButton {
            text,
            on_click
        }
    }
}

impl UIElement for UIButton {
    fn render(&self, selected: bool) -> String {
        let prefix = if selected {
            ">"
        } else {
            ""
        };
        format!("{}{}", prefix, self.text)
    }

    fn on_event(&mut self, event: GraphicsEvent) -> bool {
        match event {
            GraphicsEvent::Click => {
                (self.on_click)();
                false
            }
            _ => false,
        }
    }
}