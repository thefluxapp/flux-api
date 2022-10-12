use axum::{
  routing::get,
  Router,
};

use std::{env, net::SocketAddr, str::FromStr};

pub async fn run() {
  let app = Router::new()
  .nest(
    "/api",
    Router::new()
        .route("/status", get(|| async {}))
  );

  let addr = SocketAddr::from_str(&env::var("APP_ADDR").unwrap()).unwrap();

  axum::Server::bind(&addr)
  .serve(app.into_make_service())
  .await
  .unwrap();
}
