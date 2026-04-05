use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use esp_idf_svc::fs::littlefs::Littlefs;
use esp_idf_svc::io::vfs::MountedLittlefs;
use crate::settings::component::SettingsComponent;

const TAG: &'static str = "SettingsManager";

pub struct SettingsManager<C> {
    components: Mutex<HashMap<&'static str, Box<dyn SettingsComponent>>>,
    tx: Sender<C>
}

impl<C> SettingsManager<C> {
    pub fn new() -> Result<(Self, Receiver<C>, MountedLittlefs<Littlefs<CString>>), anyhow::Error> {
        log::info!(target: TAG, "Mounting LittleFS...");

        let mounted_littlefs = unsafe {
            let littlefs = Littlefs::new_partition("storage")?;
            MountedLittlefs::mount(littlefs, "/data")?
        };

        log::info!(target: TAG, "Filesystem usage: {:?}", mounted_littlefs.info());
        let (tx, rx) = mpsc::channel();

        Ok(
            (SettingsManager {
                components: Mutex::new(HashMap::new()),
                tx
            }, rx, mounted_littlefs)
        )
    }

    pub fn add_component<T, F>(&self, id: &'static str, f: F)
    where
        T: SettingsComponent,
        F: FnOnce(Sender<C>) -> T {
        let mut components = self.components.lock().unwrap();
        components.insert(id, Box::new(f(self.tx.clone())));
    }

    pub fn get_component<T, F, R>(
        &self,
        id: &str,
        f: F
    ) -> Option<R>
    where
        T: SettingsComponent,
        F: FnOnce(&T) -> R,
    {
        let components = self.components.lock().unwrap();

        components.get(id).and_then(|boxed| {
            boxed.as_any().downcast_ref::<T>()
        }).map(f)
    }

    pub fn direct_read(&self, id: &str, args: &Vec<&str>) -> String {
        let components = self.components.lock().unwrap();
        components.get(id).map_or(String::new(), |component| {
            component.direct_read(args)
        })
    }
}
