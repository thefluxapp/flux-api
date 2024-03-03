use axum::{extract::State, Json};
use axum_valid::Valid;

use super::{
    data::{self, CreateResponseData},
    service,
};
use crate::app::{auth::User, AppError, AppState};

// mod create;
mod messages;
mod show;

pub struct MessagesController {}

pub async fn create_message(
    State(AppState { db, notifier, .. }): State<AppState>,
    user: User,
    Valid(Json(data)): Valid<Json<data::CreateRequestData>>,
) -> Result<Json<CreateResponseData>, AppError> {
    let message = service::create_message(&db, user, data).await?;

    let text = message.text.clone();
    tokio::spawn(async move {
        notifier.send_push_notifications(message.id, text).await;
    });

    Ok(Json(message.into()))
}
