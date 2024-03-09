use axum::{extract::State, Json};

use crate::app::{auth::User, AppError, AppState};

use super::{data::IndexResponseData, service};

// pub async fn find_user_streams(
//     State(AppState { db, .. }): State<AppState>,
//     user: User,
// ) -> Result<Json<IndexResponseData>, AppError> {
//     let streams = service::find_user_streams(&db, user).await?;
//     // let message = service::create_message(&db, user, data).await?;

//     Ok(Json(streams.into()))
// }

pub async fn find_all_streams(
    State(AppState { db, .. }): State<AppState>,
    user: Option<User>,
) -> Result<Json<IndexResponseData>, AppError> {
    let (streams, streams_users) = service::find_all_streams(&db).await?;
    // let message = service::create_message(&db, user, data).await?;

    Ok(Json((streams, streams_users, user).into()))
}
