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
            Some(user) => {
                let (message, stream) =
                    MessagesService::create(&user, &state.db, request_data).await;

                let db = state.db.clone();
                let notifier = state.notifier.clone();
                let message_to_notify = message.clone();
                let user_id = user.id;

                tokio::spawn(async move {
                    notifier
                        .notify(&db, message_to_notify, stream, user_id)
                        .await;
                });

                Ok(Json(message.into()))
            }

            None => Err(AppError::Forbidden),
        }
    }
}
