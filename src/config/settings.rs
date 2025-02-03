use config::{Config, ConfigError, File};
use serde::Deserialize; // Typically "Deserialize" is used

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server_address: String,
    pub port: u16,
    pub log_level: String,
    pub error_rate: f32,
    pub response_delay_ms: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default.toml"))
            .build()?;
        config.try_deserialize()
    }
}
