mod app;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    app::run().await
}
