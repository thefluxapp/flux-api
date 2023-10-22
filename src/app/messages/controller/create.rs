use axum::{extract::State, Json};

use super::{
    super::{
        data::create::{RequestData, ResponseData},
        service::MessagesService,
    },
    MessagesController,
};
use crate::app::{AppError, AppSession, AppState};

impl MessagesController {
    pub async fn create(
        session: AppSession,
        State(state): State<AppState>,
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
