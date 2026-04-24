use crate::graphics::event::GraphicsEvent;
use crate::graphics::ui::element::{UIElement, UIElementState};

pub struct UIPageSelector {
    focus: bool,
    on_scroll: Box<dyn Fn(bool) + 'static>,
    on_format: Box<dyn Fn() -> String>,
}

impl UIPageSelector {
    pub fn new<F1, F2>(
        on_scroll: F1,
        on_format: F2,
    ) -> Self
    where
        F1: Fn(bool) + 'static,
        F2: Fn() -> String + 'static {
        Self {
            focus: false,
            on_scroll: Box::new(on_scroll),
            on_format: Box::new(on_format),
        }
    }
}

impl UIElement for UIPageSelector {
    fn render(&self, state: UIElementState) -> String {
        let prefix = if self.focus {
            "-"
        } else {
            match state {
                UIElementState::SelectorPress => "\\",
                UIElementState::Selected => ">",
                UIElementState::None => ""
            }
        };

        format!("{}Page ({})", prefix, (self.on_format)())
    }

    fn on_event(&mut self, event: GraphicsEvent) -> bool {
        match event {
            GraphicsEvent::Click => {
                self.focus = !self.focus;
                self.focus
            }
            GraphicsEvent::Back => {
                self.focus = false;
                true
            }
            GraphicsEvent::ScrollRight => {
                (self.on_scroll)(true);
                true
            }
            GraphicsEvent::ScrollLeft => {
                (self.on_scroll)(false);
                true
            }
            _ => false
        }
    }
}