//! src/configuration.rs
use config::{Config, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise the configuration reader
    let settings = Config::builder()
        .add_source(File::new("configuration.yaml", FileFormat::Yaml))
        .build()?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}
