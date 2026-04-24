use std::sync::mpsc::Sender;
use crate::graphics::event::GraphicsEvent;

pub enum NavigatorMessage {
    Navigate(String),
    Back,
    GraphicsEvent(GraphicsEvent),
    CustomEvent(u32)
}

impl NavigatorMessage {
    pub fn navigate(path: &str) -> Self {
        NavigatorMessage::Navigate(path.to_string())
    }
    pub fn back() -> Self {
        NavigatorMessage::Back
    }
    pub fn graphics_event(event: GraphicsEvent) -> Self {
        NavigatorMessage::GraphicsEvent(event)
    }
    pub fn custom_event(event: u32) -> Self {
        NavigatorMessage::CustomEvent(event)
    }
}

pub type Navigator = Sender<NavigatorMessage>;