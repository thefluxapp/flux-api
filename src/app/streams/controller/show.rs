use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

// use crate::app::{state::AppState, AppError};

use crate::app::{AppError, AppState};

use super::{
    super::{
        data::show::{RequestData, ResponseData},
        service::StreamsService,
    },
    StreamsController,
};

impl StreamsController {
    pub async fn show(
        Path(stream_id): Path<Uuid>,
        State(state): State<AppState>,
        Query(request_data): Query<RequestData>,
    ) -> Result<Json<ResponseData>, AppError> {
        request_data.validate().unwrap();

        let (stream, messages, users, streams) =
            StreamsService::show(&state.db, stream_id, request_data).await?;

        Ok(Json((stream, messages, users, streams).into()))
    }
}
