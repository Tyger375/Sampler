#[derive(Debug)]
pub enum GraphicsEvent {
    Click,
    Back,

    ScrollLeft,
    ScrollRight,

    ScreenStart,
    ScreenEnd
}