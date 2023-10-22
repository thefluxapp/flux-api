use axum::{routing::post, Router};

use self::controller::MessagesController;

use super::AppState;

mod controller;
pub mod data;
pub mod entities;
pub mod repo;
mod service;

// use axum::{
//     routing::{get, post},
//     Router,
// };

// use self::controller::MessagesController;

pub fn router() -> Router<AppState> {
    Router::new()
        // .route("/", get(MessagesController::index))
        .route("/", post(MessagesController::create))
}
