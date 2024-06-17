use std::{env, fs};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ObservabilitySettings {
    pub endpoint: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub endpoint: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Deserialize, Debug)]
pub struct HttpSettings {
    pub endpoint: String,
}

#[derive(Deserialize, Debug)]
pub struct NotifierSettings {
    pub endpoint: String,
}

#[derive(Deserialize, Debug)]
pub struct YaGPTSettings {
    pub completion_url: String,
    pub operation_url: String,
    pub model_url: String,
    pub api_key: String,
    pub temperature: f32,
    pub instruction: String,
    pub max_tokens: i32,
}

#[derive(Deserialize, Debug)]
pub struct AuthSettings {
    pub private_key_file: String,
    pub private_key: Vec<u8>,

    pub public_key_file: String,
    pub public_key: Vec<u8>,

    pub rp_id: String,
    pub rp_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub name: String,
    // TODO: make enum for env
    pub env: String,
    pub http: HttpSettings,
    pub auth: AuthSettings,
    pub observability: ObservabilitySettings,
    pub database: DatabaseSettings,
    pub notifier: NotifierSettings,
    pub ya_gpt: YaGPTSettings,
}

pub fn new() -> Result<Settings, ConfigError> {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let app_dir = env::var("APP_DIR").unwrap_or_else(|_| ".".into());

    let s = Config::builder()
        .add_source(File::with_name(&format!("{}/config/default", app_dir)))
        .add_source(File::with_name(&format!("{}/config/{}", app_dir, app_env)).required(false))
        .add_source(File::with_name(&format!("{}/config/local", app_dir)).required(false))
        .add_source(Environment::with_prefix("app").separator("_"))
        .set_default("env", app_env)?
        .set_default(
            "auth.private_key",
            read_env_key_from_file("AUTH_PRIVATE_KEY_FILE"),
        )?
        .set_default(
            "auth.public_key",
            read_env_key_from_file("AUTH_PUBLIC_KEY_FILE"),
        )?;

    s.build()?.try_deserialize()
}

fn read_env_key_from_file(key: &str) -> Vec<u8> {
    match env::var(key) {
        Ok(file_path) => fs::read_to_string(file_path)
            .unwrap_or_default()
            .into_bytes(),
        Err(_) => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_env() {
        let v: Vec<u8> = vec![];
        assert_eq!(read_env_key_from_file("NOT_EXISTS"), v);
    }

    #[test]
    fn without_file() {
        let v: Vec<u8> = vec![];

        env::set_var("KEY", "not_exists_path");

        assert_eq!(read_env_key_from_file("KEY"), v);
    }
}
