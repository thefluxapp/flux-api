use dotenv::dotenv;

mod app;
mod tracing;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing::run().await;

    app::run().await;
}
