use crate::graphics::{event::GraphicsEvent, ui::element::UIElement};
use crate::graphics::ui::element::UIElementState;

pub struct UIButton {
    text: String,
    on_click: Box<dyn Fn()>
}

impl UIButton {
    pub fn new<F>(text: String, on_click: F) -> Self
    where F: Fn() + 'static {
        UIButton {
            text,
            on_click: Box::new(on_click)
        }
    }
}

impl UIElement for UIButton {
    fn render(&self, state: UIElementState) -> String {
        let prefix = match state {
            UIElementState::Selected => ">",
            UIElementState::SelectorPress => "\\",
            UIElementState::None => ""
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