use dotenv::dotenv;
use tracing_subscriber;

mod app;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().pretty().init();

    app::run().await;
}
