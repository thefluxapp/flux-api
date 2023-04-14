use dotenv::dotenv;

mod app;
mod tracing;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing::run();

    app::run().await;
}
