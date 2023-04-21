use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::controllers::SessionControllers;

mod controllers;
mod data;
mod entities;
pub mod middleware;
mod services;

pub fn router() -> Router {
    Router::new()
        .route("/", get(SessionControllers::show))
        .route("/auth", post(SessionControllers::auth))
}

#[derive(Debug, Deserialize)]
struct JwtUser {
    pub sub: Uuid,
}

/// Contains session info
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub user: Option<entities::user::Model>,
}
