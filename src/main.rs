use dotenv::dotenv;
use tracing_subscriber::{self, EnvFilter};

mod app;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    app::run().await;
}
