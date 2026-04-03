use std::any::Any;
use std::sync::mpsc::Sender;

pub trait SettingsComponent: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}