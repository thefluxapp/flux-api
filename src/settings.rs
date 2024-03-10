use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Observability {
    pub endpoint: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Settings {
    pub name: String,
    // TODO: make enum for env
    pub env: String,
    pub observability: Observability,
}

pub fn new() -> Result<Settings, ConfigError> {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let app_dir = env::var("APP_DIR").unwrap_or_else(|_| ".".into());

    let s = Config::builder()
        .add_source(File::with_name(&format!("{}/config/default", app_dir)))
        .add_source(File::with_name(&format!("{}/config/{}", app_dir, app_env)).required(false))
        .add_source(Environment::with_prefix("app"))
        .set_override("env", app_env)?
        .build()?;

    s.try_deserialize()
}
