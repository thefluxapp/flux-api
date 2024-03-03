use axum::{
    routing::{get, post},
    Router,
};

use self::controller::MessagesController;

use super::AppState;

mod controller;
pub mod data;
pub mod entities;
pub mod repo;
mod service;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(controller::create_message))
        .route("/:message_id", get(MessagesController::show))
        .route("/:message_id/messages", get(MessagesController::messages))
}
