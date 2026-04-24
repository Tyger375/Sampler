#[derive(Debug)]
pub enum GraphicsEvent {
    Click,
    Back,

    ScrollLeft,
    ScrollRight,

    ScreenStart,
    ScreenEnd,

    Refresh(String)
}

impl GraphicsEvent {
    pub fn refresh(message: &str) -> Self {
        GraphicsEvent::Refresh(message.to_string())
    }
}