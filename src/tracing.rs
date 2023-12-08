use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::Resource;
use std::env;
use tracing_subscriber::Layer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

enum Tracing {
    Otel,
    Stdout,
}

pub async fn run() {
    let tracing = match env::var("APP_TRACING") {
        Ok(val) => match val.as_str() {
            "otel" => Tracing::Otel,
            _ => Tracing::Stdout,
        },
        Err(_) => Tracing::Stdout,
    };

    match tracing {
        Tracing::Otel => {
            let logger = opentelemetry_otlp::new_pipeline()
                .logging()
                .with_log_config(
                    opentelemetry_sdk::logs::config().with_resource(Resource::new(vec![
                        KeyValue::new("service.name", "flux-api"),
                        KeyValue::new(
                            "deployment.environment",
                            env::var("APP_ENV").unwrap_or("local".to_string()),
                        ),
                    ])),
                )
                .with_exporter(opentelemetry_otlp::new_exporter().tonic())
                .install_batch(opentelemetry_sdk::runtime::Tokio)
                .unwrap();

            let logger = OpenTelemetryTracingBridge::new(&logger.provider().unwrap())
                .with_filter(EnvFilter::from_default_env());

            let tracer = tracing_opentelemetry::layer().with_tracer(
                opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(opentelemetry_otlp::new_exporter().tonic())
                    .install_batch(opentelemetry_sdk::runtime::Tokio)
                    .unwrap(),
            );

            let subscriber = tracing_subscriber::registry().with(tracer).with(logger);
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
        Tracing::Stdout => {
            tracing_subscriber::fmt()
                .with_thread_names(true)
                .with_env_filter(EnvFilter::from_default_env())
                .pretty()
                .init();
        }
    };
}
