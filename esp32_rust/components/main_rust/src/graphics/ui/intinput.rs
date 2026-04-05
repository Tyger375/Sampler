use crate::graphics::event::GraphicsEvent;
use crate::graphics::ui::element::{UIElement, UIElementState};

pub struct IntInputConfig {
    pub text: String,
    pub format_value: Box<dyn Fn(i32) -> String>,
    pub on_change: Box<dyn Fn(i32) -> i32>,
    pub on_done: Box<dyn Fn(i32) -> ()>
}

impl Default for IntInputConfig {
    fn default() -> Self {
        IntInputConfig {
            text: "Input".to_string(),
            format_value: Box::new(|value| value.to_string()),
            on_change: Box::new(|value| value),
            on_done: Box::new(|_| {})
        }
    }
}

pub struct UIIntInput {
    focus: bool,
    config: IntInputConfig,
    old_value: i32,
    value: i32,
}

impl UIIntInput {
    pub fn new(config: IntInputConfig, default_value: i32) -> Self {
        UIIntInput {
            focus: false,
            config,
            old_value: default_value,
            value: default_value
        }
    }
}

impl UIElement for UIIntInput {
    fn render(&self, state: UIElementState) -> String {
        let prefix = match state {
            UIElementState::Selected => ">",
            UIElementState::SelectorPress => "\\",
            UIElementState::None => ""
        };
        format!("{}{}: {}", prefix, self.config.text, (self.config.format_value)(self.value))
    }

    fn on_event(&mut self, event: GraphicsEvent) -> bool {
        match event {
            GraphicsEvent::Click => {
                self.focus = !self.focus;
                if !self.focus && self.value != self.old_value {
                    (self.config.on_done)(self.value);
                }
                self.old_value = self.value;
                self.focus
            }
            GraphicsEvent::Back => {
                self.value = self.old_value;
                self.focus
            }
            GraphicsEvent::ScrollRight => {
                self.value = (self.config.on_change)(self.value + 1);
                false
            }
            GraphicsEvent::ScrollLeft => {
                self.value = (self.config.on_change)(self.value - 1);
                false
            }
            _ => false
        }
    }
}