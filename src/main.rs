mod app;
mod observability;
mod settings;

#[tokio::main]
async fn main() {
    // load settings
    let settings = settings::new().unwrap();

    // start logs, _traces, _metrics
    observability::run(&settings);

    // start main app module
    app::run(&settings).await;
}
