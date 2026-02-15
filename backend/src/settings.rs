use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // 1. Configuración base (default.toml)
            .add_source(File::with_name("config/default"))
            // 2. Configuración por entorno (opcional, ej: config/production.toml)
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // 3. Variables de entorno (ej: APP_PORT=8080 sobrescribe port)
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize()
    }
}