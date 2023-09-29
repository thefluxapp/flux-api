use axum::{
    routing::{get, post},
    Router,
};

use self::controller::AuthController;

use super::AppState;

mod controller;
mod data;
mod entities;
mod middleware;
mod repo;
mod service;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(AuthController::login))
        .route("/join", post(AuthController::join))
        .route("/complete", post(AuthController::complete))
        .route("/me", get(AuthController::me))
}
