use crate::app::state::AppState;
use axum::{Extension, Json};
use std::sync::Arc;

use super::super::{data::index::ResponseData, services::StreamsServices};
use super::StreamsControllers;

impl StreamsControllers {
    pub async fn index(state: Extension<Arc<AppState>>) -> Json<ResponseData> {
        Json(StreamsServices::index(&state.db).await.into())
    }
}
