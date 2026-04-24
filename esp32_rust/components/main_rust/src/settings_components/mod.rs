pub mod config;
pub mod pads;
pub mod shortcuts;
pub mod sequencer;
pub mod sequencer_projects;

pub enum SettingsEvent {
    ConfigBpm,
    PadConfig
}
