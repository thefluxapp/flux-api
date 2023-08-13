use axum::{Extension, Json};
use std::sync::Arc;

use super::{
    super::data::create::{RequestData, ResponseData},
    MessagesController,
};
use crate::app::{messages::service::MessagesService, session::Session, state::AppState, AppError};

impl MessagesController {
    pub async fn create(
        session: Session,
        state: Extension<Arc<AppState>>,
        Json(request_data): Json<RequestData>,
    ) -> Result<Json<ResponseData>, AppError> {
        match session.user {
            Some(user) => Ok(Json(
                MessagesService::create(&user, &state.db, request_data)
                    .await
                    .into(),
            )),
            None => Err(AppError::Forbidden),
        }
    }
}
