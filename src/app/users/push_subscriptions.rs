use axum::{
    routing::{get, post},
    Router,
};

use crate::app::AppState;

use self::controller::PushSubscriptionsController;

mod controller;
mod data;
mod entities;
mod repo;
mod service;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(PushSubscriptionsController::create))
        .route("/vapid", get(PushSubscriptionsController::vapid))
        .route("/test", get(PushSubscriptionsController::test))
}
