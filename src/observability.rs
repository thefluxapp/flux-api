use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_otlp::WithExportConfig as _;
use opentelemetry_sdk::{logs::Config, Resource};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::Settings;

pub fn run(settings: &Settings) {
    let config = Config::default().with_resource(Resource::new(vec![
        KeyValue::new("service.name", settings.name.clone()),
        KeyValue::new("deployment.environment", settings.env.clone()),
    ]));

    match settings.env.as_str() {
        "production" => {
            let logger = opentelemetry_otlp::new_pipeline()
                .logging()
                .with_log_config(config)
                .with_exporter(
                    opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint(settings.observability.endpoint.clone()),
                )
                .install_batch(opentelemetry_sdk::runtime::Tokio)
                .unwrap();

            let layer = layer::OpenTelemetryTracingBridge::new(logger.provider());
            tracing_subscriber::registry()
                .with(tracing_subscriber::filter::LevelFilter::from_level(
                    Level::INFO,
                ))
                .with(layer)
                .init();
        }
        _ => {
            tracing_subscriber::fmt().init();
        }
    };
}
