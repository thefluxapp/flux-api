use axum::{extract::State, Json};
use axum_valid::Valid;

use crate::app::{AppError, AppState};

use super::{
    data::{
        CompleteRequestData, CompleteResponseData, JoinRequestData, JoinResponseData,
        LoginRequestData, LoginResponseData, MeResponseData,
    },
    service, User,
};

pub async fn join(
    State(AppState { db, auth_state, .. }): State<AppState>,
    Valid(Json(data)): Valid<Json<JoinRequestData>>,
) -> Result<Json<JoinResponseData>, AppError> {
    Ok(Json(service::join(&db, &auth_state, data.email).await?))
}

pub async fn login(
    State(AppState { db, auth_state, .. }): State<AppState>,
    Valid(Json(data)): Valid<Json<LoginRequestData>>,
) -> Result<Json<LoginResponseData>, AppError> {
    let user = service::login(&db, &auth_state, data).await?;
    let jwt = service::create_jwt(&auth_state, &user)?;

    Ok(Json((user, jwt).into()))
}

pub async fn complete(
    State(AppState { db, auth_state, .. }): State<AppState>,
    Valid(Json(data)): Valid<Json<CompleteRequestData>>,
) -> Result<Json<CompleteResponseData>, AppError> {
    let user = service::complete(&db, &auth_state, data).await?;
    let jwt = service::create_jwt(&auth_state, &user)?;

    Ok(Json((user, jwt).into()))
}

pub async fn me(
    user: Option<User>,
) -> Result<Json<MeResponseData>, AppError> {
    // let user: Option<User> = None;

    Ok(Json(user.into()))
}
