use std::sync::mpsc::Sender;
use crate::graphics::event::GraphicsEvent;

pub enum NavigatorMessage {
    Navigate(String),
    Back,
    GraphicsEvent(GraphicsEvent)
}

pub type Navigator = Sender<NavigatorMessage>;