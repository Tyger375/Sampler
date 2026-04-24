use crate::graphics::event::GraphicsEvent;
use crate::graphics::ui::element::{UIElement, UIElementState};

pub struct IntInputConfig {
    pub text: String,
    pub format_value: Box<dyn Fn(i32) -> String>,
    pub on_change: Box<dyn Fn(i32, i32) -> i32>,
    pub on_done: Box<dyn Fn(i32) -> ()>
}

impl IntInputConfig {
    pub fn new<F1, F2, F3>(
        text: &str,
        format_value: F1,
        on_change: F2,
        on_done: F3
    ) -> Self
    where
        F1: Fn(i32) -> String + 'static,
        F2: Fn(i32, i32) -> i32    + 'static,
        F3: Fn(i32) -> ()     + 'static {
        Self {
            text: String::from(text),
            format_value: Box::new(format_value),
            on_change: Box::new(on_change),
            on_done: Box::new(on_done)
        }
    }
}

impl Default for IntInputConfig {
    fn default() -> Self {
        IntInputConfig {
            text: "Input".to_string(),
            format_value: Box::new(|value| value.to_string()),
            on_change: Box::new(|value, _| value),
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
            UIElementState::Selected => if self.focus {
                "-"
            } else {
                ">"
            },
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
                self.focus = false;
                self.value = self.old_value;
                true
            }
            GraphicsEvent::ScrollRight => {
                self.value = (self.config.on_change)(self.value + 1, self.value);
                false
            }
            GraphicsEvent::ScrollLeft => {
                self.value = (self.config.on_change)(self.value - 1, self.value);
                false
            }
            _ => false
        }
    }
}