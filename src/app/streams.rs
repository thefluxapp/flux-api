use axum::{routing::get, Router};

use super::AppState;

pub mod controller;
pub mod data;
pub mod entities;
pub mod repo;
pub mod service;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(controller::find_all_streams))
    // .route("/all", get(controller::find_all_streams))
}
