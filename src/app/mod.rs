use axum::{routing::get, routing::post, Extension, Router};
use std::{env, net::SocketAddr, str::FromStr};

mod db;
mod sessions;
mod users;

pub async fn run() {
    let pool = db::create_pool(&env::var("DATABASE_URL").unwrap()).await;

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/status", get(|| async {}))
                .route("/session", get(sessions::show))
                .route("/session/auth", post(sessions::auth)),
        )
        .layer(Extension(pool));

    let addr = SocketAddr::from_str(&env::var("APP_ADDR").unwrap()).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
