use axum::extract::State;
// use crate::app::state::AppState;
use axum::Json;

use crate::app::AppState;

use super::super::{data::index::ResponseData, service::StreamsService};
use super::StreamsController;

impl StreamsController {
    pub async fn index(State(state): State<AppState>) -> Json<ResponseData> {
        let streams = StreamsService::index(&state.db).await;

        Json(streams.into())
    }
}
