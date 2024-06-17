use axum::{extract::State, Json};
use axum_valid::Valid;

use crate::app::{AppError, AppState};

use super::{
    data::{
        self, CompleteRequestData, JoinRequestData, JoinResponseData, LoginRequestData,
        MeResponseData,
    },
    service, User,
};

// mod complete;
// mod index;
// mod join;
// mod login;

pub struct AuthController;

pub async fn join(
    State(AppState { db, auth_state, .. }): State<AppState>,
    // user: User,
    Valid(Json(data)): Valid<Json<JoinRequestData>>,
) -> Result<Json<JoinResponseData>, AppError> {
    Ok(Json(service::join(&db, &auth_state, data.email).await?))
}

pub async fn login(
    State(AppState { db, .. }): State<AppState>,
    Valid(Json(data)): Valid<Json<LoginRequestData>>,
) -> Result<String, AppError> {
    dbg!(&data);

    service::login(&db, data).await?;

    Ok("LOGIN".to_string())
}

pub async fn complete(
    State(AppState { db, .. }): State<AppState>,
    Valid(Json(data)): Valid<Json<CompleteRequestData>>,
) -> Result<String, AppError> {
    dbg!(&data);

    // let pk = ring::signature::UnparsedPublicKey::new(
    //     &ring::signature::ECDSA_P256_SHA256_ASN1,
    //     "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExcZba0ZbZGeZ5IFwZgxcNeXTfzC21e8hcN2JnYzftN_o0taxMKQnDUoYeOL2C92s0jOo5JQGbdxbtDDhGeW6BA",
    // );

    // let client_data_json_hash =
    //     ring::digest::digest(&ring::digest::SHA256, data.client_data_json.as_bytes());

    // let verification_data: Vec<u8> = data
    //     .authenticator_data
    //     .as_bytes()
    //     .iter()
    //     .chain(client_data_json_hash.as_ref().iter())
    //     .copied()
    //     .collect();

    // let qq = pk.verify(&verification_data, data.signature.as_bytes());

    // dbg!(&qq);

    service::complete(&db, data).await?;

    Ok("COMPLETE".to_string())
}

pub async fn me() -> Result<Json<MeResponseData>, AppError> {
    let user: Option<User> = None;

    Ok(Json(user.into()))
}
