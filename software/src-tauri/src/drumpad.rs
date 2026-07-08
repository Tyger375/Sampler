use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PadConfig {
    pub id: u8,
    pub note: u8,
    pub track_id: usize
}

impl Default for PadConfig {
    fn default() -> Self {
        Self {
            id: 0,
            note: 60,
            track_id: 0
        }
    }
}