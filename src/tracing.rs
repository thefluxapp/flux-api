use std::env;
use tracing_subscriber::EnvFilter;

pub async fn run() {
    let is_production =
        env::var("APP_ENV").unwrap_or(String::from("development")) == String::from("production");

    if is_production {
        tracing_subscriber::fmt()
            .with_thread_names(true)
            .with_env_filter(EnvFilter::from_default_env())
            .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_thread_names(true)
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .init();
    };
}
