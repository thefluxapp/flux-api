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
    State(AppState { db, .. }): State<AppState>,
    user: User,
    Valid(Json(data)): Valid<Json<data::CreateRequestData>>,
) -> Result<Json<CreateResponseData>, AppError> {
    let message = service::create_message(&db, user, data).await?;

    Ok(Json(message.into()))
}
