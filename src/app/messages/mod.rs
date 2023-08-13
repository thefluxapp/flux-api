mod controller;
pub mod data;
pub mod entities;
mod repo;
mod service;

use axum::{
    routing::{get, post},
    Router,
};

use self::controller::MessagesController;

pub fn router() -> Router {
    Router::new()
        .route("/", get(MessagesController::index))
        .route("/", post(MessagesController::create))
}
