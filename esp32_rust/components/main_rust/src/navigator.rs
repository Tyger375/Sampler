use std::sync::mpsc::Sender;
use crate::graphics::event::GraphicsEvent;

pub enum NavigatorMessage {
    Navigate(String),
    Back,
    GraphicsEvent(GraphicsEvent),
    CustomEvent(u32)
}

pub type Navigator = Sender<NavigatorMessage>;