use dotenv::dotenv;

mod app;
mod observability;
mod settings;

#[tokio::main]
async fn main() {
    // load settings
    let settings = settings::new().unwrap();

    // legacy, need to remove in favor of settings
    dotenv().ok();

    // start logs, _traces, _metrics
    observability::run(&settings);

    // start main app module
    app::run().await;
}
