use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Extension, Router};
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;
use std::{env, net::SocketAddr, str::FromStr};
use tracing::info;

use self::state::AppState;

mod db;
mod state;

mod messages;
mod session;
mod streams;
mod tasks;
mod users;

pub async fn run() {
    let state = Arc::new(AppState {
        db: db::create_pool(&env::var("DATABASE_URL").unwrap()).await,
    });

    // TODO: Deal with it later
    Migrator::up(&state.db, None).await.unwrap();
    info!("Migrator finished");

    // Start tasks processor
    tasks::executor::run(&state).await;

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/healthz", get(|| async {}))
                .nest("/session", session::router())
                .nest("/messages", messages::router())
                .nest("/streams", streams::router()),
        )
        .layer(Extension(state));

    let addr = SocketAddr::from_str(&env::var("APP_ADDR").unwrap()).unwrap();

    info!("App start on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub enum AppError {
    EntityNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::EntityNotFound => StatusCode::NOT_FOUND,
        };

        (status).into_response()
    }
}
