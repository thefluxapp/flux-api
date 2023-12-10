use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use super::{
    super::{data::show::ResponseData, service::MessagesService},
    MessagesController,
};
use crate::app::{AppError, AppState};

impl MessagesController {
    pub async fn show(
        Path(message_id): Path<Uuid>,
        State(state): State<AppState>,
    ) -> Result<Json<ResponseData>, AppError> {
        match MessagesService::show(&state.db, message_id).await {
            Some(data) => Ok(Json(data.into())),
            None => Err(AppError::EntityNotFound),
        }
    }
}
