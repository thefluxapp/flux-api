use axum::{routing::get, Router};

use self::controllers::StreamsControllers;

pub mod controllers;
pub mod data;
pub mod entities;
pub mod services;

pub fn router() -> Router {
    Router::new()
        .route("/", get(StreamsControllers::index))
        .route("/:stream_id", get(StreamsControllers::show))
}
