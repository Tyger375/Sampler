use std::cmp::min;
use crate::graphics::event::GraphicsEvent;
use crate::graphics::ui::element::{UIElement, UIElementState};

pub struct UIRow {
    label: String,
    elements: Vec<Box<dyn UIElement>>,
    offset: usize,
    focus: bool,
}

impl UIRow {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            elements: vec![],
            offset: 0,
            focus: false
        }
    }
    pub fn with_label(label: &str) -> Self {
        Self {
            label: format!("{}: ", label),
            elements: vec![],
            offset: 0,
            focus: false
        }
    }

    pub fn add_element<T>(&mut self, item: T)
    where T: UIElement + 'static {
        self.elements.push(Box::new(item));
    }
}

impl UIElement for UIRow {
    fn render(&self, state: UIElementState) -> String {
        let prefix = match state {
            UIElementState::Selected => ">",
            UIElementState::SelectorPress => "\\",
            UIElementState::None => ""
        };

        let mut text = format!("{}{}", prefix, self.label);

        if let Some(element) = self.elements.get(self.offset) {
            let child_state = match state {
                UIElementState::SelectorPress => UIElementState::SelectorPress,
                _ if self.focus => UIElementState::Selected,
                _ => UIElementState::None
            };

            text.push_str(&element.render(child_state));
            text.push(' ');
        }
        if let Some(element) = self.elements.get(self.offset + 1) {
            text.push_str(element.render(UIElementState::None).as_str());
        }

        let rendered = format!("{:<16}", text);
        rendered.chars().take(16).collect()
    }

    fn on_event(&mut self, event: GraphicsEvent) -> bool {
        match event {
            GraphicsEvent::Click => {
                if !self.focus {
                    self.focus = true
                } else {
                    return self.elements[self.offset].on_event(event);
                }
                true
            }
            GraphicsEvent::Back => {
                if self.focus {
                    self.focus = false;
                    true
                } else {
                    false
                }
            }
            GraphicsEvent::ScrollRight => {
                self.offset = min(self.offset + 1, self.elements.len() - 1);
                false
            }
            GraphicsEvent::ScrollLeft => {
                if self.offset > 0 {
                    self.offset -= 1;
                }
                false
            }
            _ => false
        }
    }
}