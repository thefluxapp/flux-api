use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use self::data::{AuthData, SessionData};

mod data;
mod service;
mod entities;

pub async fn auth(Extension(pool): Extension<DatabaseConnection>) -> Json<AuthData> {
    Json(service::auth(&pool).await)
}

pub async fn show(Extension(pool): Extension<DatabaseConnection>) -> Json<SessionData> {
    Json(service::show(&pool))
}
