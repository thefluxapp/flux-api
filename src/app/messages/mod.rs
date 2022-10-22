mod data;
mod entities;
mod payload;
mod service;

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use sea_orm::DatabaseConnection;

use self::{
    data::{CreateData, IndexData},
    payload::CreateMessagePayload,
    service::MessagesService,
};
use super::User;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/", post(create))
}

async fn index() -> Json<IndexData> {
    Json(MessagesService::index())
}

async fn create(
    user: User,
    Extension(pool): Extension<DatabaseConnection>,
    Json(payload): Json<CreateMessagePayload>,
) -> Json<CreateData> {
    Json(MessagesService::create(user, &pool, payload).await)
}
