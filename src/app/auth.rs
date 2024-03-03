use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    routing::{get, post},
    BoxError, RequestPartsExt, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use uuid::Uuid;

use self::controller::AuthController;
use super::{users::repo as users_repo, AppError, AppState, JwtUser};

mod controller;
mod data;
mod entities;
mod repo;
mod service;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(AuthController::login))
        .route("/join", post(AuthController::join))
        .route("/complete", post(AuthController::complete))
        .route("/", get(AuthController::index))
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub image: Option<String>,
}

impl From<entities::user::Model> for User {
    fn from(user: entities::user::Model) -> Self {
        User {
            id: user.id,
            name: user.name(),
            image: user.image(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AppState {
            db,
            auth_public_key,
            ..
        } = AppState::from_ref(state);

        let user = get_user_from_jwt(parts, &db, &auth_public_key)
            .await
            .map_err(|_| AppError::Forbidden)?
            .ok_or(AppError::Forbidden)?;

        Ok(user.into())
    }
}

pub async fn get_user_from_jwt(
    parts: &mut Parts,
    db: &DatabaseConnection,
    auth_public_key: &Vec<u8>,
) -> Result<Option<entities::user::Model>, BoxError> {
    let jwt_user = get_jwt_user(parts, auth_public_key).await?;
    let user = users_repo::find_by_id(db, jwt_user.sub).await;

    Ok(user)
}

async fn get_jwt_user(parts: &mut Parts, auth_public_key: &Vec<u8>) -> Result<JwtUser, BoxError> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await?;

    let TokenData { claims, .. } = decode::<JwtUser>(
        bearer.token(),
        &DecodingKey::from_rsa_pem(auth_public_key).unwrap(),
        &Validation::new(Algorithm::RS256),
    )?;

    Ok(claims)
}
