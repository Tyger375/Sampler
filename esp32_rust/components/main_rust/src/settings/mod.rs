use std::fs;
use std::io::Write;
use serde::{Serialize};
use serde::de::DeserializeOwned;

pub mod manager;
pub mod component;

pub fn load_config<T>(path: &str) -> T
where T: Serialize + DeserializeOwned {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = toml::from_str::<T>(&content) {
            return config
        }
    }
    panic!("Config not found!");
}

pub fn load_config_or_default<T>(path: &str) -> T
where T: Serialize + DeserializeOwned + Default {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = toml::from_str::<T>(&content) {
            return config;
        }
    }

    let default_conf = T::default();
    save_config(path, &default_conf);
    default_conf
}

pub fn load_config_or_null<T>(path: &str) -> Option<T>
where T: Serialize + DeserializeOwned {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = toml::from_str::<T>(&content) {
            return Some(config);
        }
    }
    None
}

pub fn save_config<T>(path: &str, config: &T)
where T: Serialize {
    if let Ok(toml_str) = toml::to_string_pretty(config) {
        if let Ok(mut file) = fs::File::create(path) {
            let _ = file.write_all(toml_str.as_bytes());
        }
    }
}