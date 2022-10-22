mod data;
mod entities;
mod service;

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use sea_orm::DatabaseConnection;

use self::{data::{IndexData, CreateData}, service::MessagesService};
use super::User;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/", post(create))
}

async fn index() -> Json<IndexData> {
    Json(MessagesService::index())
}

async fn create(user: User, Extension(pool): Extension<DatabaseConnection>) -> Json<CreateData> {
    Json(MessagesService::create(user, &pool).await)
}
