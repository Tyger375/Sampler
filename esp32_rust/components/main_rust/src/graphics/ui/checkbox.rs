use crate::graphics::event::GraphicsEvent;
use crate::graphics::ui::element::{UIElement, UIElementState};

pub struct UICheckBox {
    label: String,
    on_change: Box<dyn Fn(bool) -> bool>,
    checked: bool,
}

impl UICheckBox {
    pub fn new<F>(
        label: String,
        on_change: F,
        default_value: bool
    ) -> Self
    where F: Fn(bool) -> bool + 'static {
        Self {
            label,
            on_change: Box::new(on_change),
            checked: default_value
        }
    }
}

impl UIElement for UICheckBox {
    fn render(&self, state: UIElementState) -> String {
        let prefix = match state {
            UIElementState::Selected => ">",
            UIElementState::SelectorPress => "\\",
            UIElementState::None => ""
        };
        let checked = if self.checked {
            "ON"
        } else {
            "OFF"
        };
        format!("{prefix}{}: {checked}", self.label)
    }

    fn on_event(&mut self, event: GraphicsEvent) -> bool {
        match event {
            GraphicsEvent::Click => {
                self.checked = (self.on_change)(!self.checked);
            }
            _ => ()
        }
        false
    }
}