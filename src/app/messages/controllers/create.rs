use axum::{http::StatusCode, Extension, Json};
use std::sync::Arc;

use super::{
    super::data::create::{RequestData, ResponseData},
    MessagesControllers,
};
use crate::app::{messages::services::MessagesServices, session::Session, state::AppState};

impl MessagesControllers {
    pub async fn create(
        session: Session,
        state: Extension<Arc<AppState>>,
        Json(request_data): Json<RequestData>,
    ) -> Result<Json<ResponseData>, StatusCode> {
        match session.user {
            Some(user) => Ok(Json(
                MessagesServices::create(&user, &state.db, request_data)
                    .await
                    .into(),
            )),
            None => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
