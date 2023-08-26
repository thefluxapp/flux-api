use axum::{routing::get, Router};

use self::controller::StreamsController;

use super::AppState;

// use self::controllers::StreamsControllers;

pub mod controller;
pub mod data;
pub mod entities;
pub mod repo;
pub mod service;
// pub mod services;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(StreamsController::index))
        .route("/:stream_id", get(StreamsController::show))
}
