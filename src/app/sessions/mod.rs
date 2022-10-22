use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use self::data::{AuthData, SessionData};

use super::User;

mod data;
mod entities;
pub mod service;

pub async fn auth(Extension(pool): Extension<DatabaseConnection>) -> Json<AuthData> {
    Json(service::auth(&pool).await)
}

pub async fn show(user: User, Extension(pool): Extension<DatabaseConnection>) -> Json<SessionData> {
    Json(service::show(user, &pool))
}
