use crate::graphics::event::GraphicsEvent;

pub type BoxedUIElement = Box<dyn UIElement>;

pub trait UIElement {
    fn render(&self, selected: bool) -> String;

    fn on_event(&mut self, _event: GraphicsEvent) -> bool {
        false
    }
}