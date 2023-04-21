pub mod data;
pub mod entities;
mod services;

use axum::{
    routing::{get, post},
    Router,
};

use self::controllers::MessagesControllers;
mod controllers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(MessagesControllers::index))
        .route("/", post(MessagesControllers::create))
}
