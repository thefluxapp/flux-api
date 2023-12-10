use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use super::{
    super::{
        data::messages::{RequestData, ResponseData},
        service::MessagesService,
    },
    MessagesController,
};
use crate::app::AppState;

impl MessagesController {
    pub async fn messages(
        State(state): State<AppState>,
        Path(message_id): Path<Uuid>,
        Query(request_data): Query<RequestData>,
    ) -> Json<ResponseData> {
        Json(
            MessagesService::messages(&state.db, message_id, request_data)
                .await
                .into(),
        )
    }
}
