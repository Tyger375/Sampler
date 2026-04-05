use crate::graphics::event::GraphicsEvent;

pub type BoxedUIElement = Box<dyn UIElement>;

pub enum UIElementState {
    Selected,
    SelectorPress,
    None
}

pub trait UIElement {
    fn render(&self, state: UIElementState) -> String;

    fn on_event(&mut self, _event: GraphicsEvent) -> bool {
        false
    }
}