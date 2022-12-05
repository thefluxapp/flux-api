use axum::{extract::Path, routing::get, Extension, Json, Router};
use sea_orm::{prelude::Uuid, DatabaseConnection};
use tokenizers::Tokenizer;
use std::sync::Arc;

use self::{
    data::{StreamsIndexData, StreamsShowData},
    service::StreamsService,
};

pub mod data;
pub mod entities;
pub mod service;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/:stream_id", get(show))
}

async fn index(Extension(pool): Extension<DatabaseConnection>) -> Json<StreamsIndexData> {
    Json(StreamsService::index(&pool).await.into())
}

async fn show(
    Path(stream_id): Path<Uuid>,
    tokenizer: Extension<Arc<Tokenizer>>,
    pool: Extension<Arc<DatabaseConnection>>,
) -> Json<StreamsShowData> {
    Json(StreamsService::show(stream_id, &pool, &tokenizer).await.into())
}
