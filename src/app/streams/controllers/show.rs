use axum::{extract::Path, Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

use crate::app::{state::AppState, AppError};

use super::{
    super::{data::show::ResponseData, services::StreamsServices},
    StreamsControllers,
};

impl StreamsControllers {
    pub async fn show(
        Path(stream_id): Path<Uuid>,
        state: Extension<Arc<AppState>>,
    ) -> Result<Json<ResponseData>, AppError> {
        let (stream, messages) = StreamsServices::show(stream_id, &state.db).await?;

        Ok(Json((stream, messages).into()))
    }
}
