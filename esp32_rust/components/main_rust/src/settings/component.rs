use std::any::Any;

pub trait SettingsComponent: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;

    fn direct_read(&self, args: &Vec<&str>) -> String;
}