use axum::{
    routing::{get, post},
    Router,
};

use self::controller::SessionController;

mod controller;
mod data;
mod entities;
mod service;

pub fn router() -> Router {
    Router::new()
        .route("/", get(SessionController::show))
        .route("/auth", post(SessionController::auth))
}
