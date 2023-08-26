use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

// use crate::app::{state::AppState, AppError};

use crate::app::{AppError, AppState};

use super::{
    super::{data::show::ResponseData, service::StreamsService},
    StreamsController,
};

impl StreamsController {
    pub async fn show(
        Path(stream_id): Path<Uuid>,
        State(state): State<AppState>,
    ) -> Result<Json<ResponseData>, AppError> {
        let (stream, messages) = StreamsService::show(&state.db, stream_id).await?;

        Ok(Json((stream, messages).into()))
    }
}
