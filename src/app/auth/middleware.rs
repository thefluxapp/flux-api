use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::Response,
    BoxError, RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::app::{AppSession, JwtUser};

use super::{super::AppState, entities};

#[async_trait]
impl<S> FromRequestParts<S> for AppSession
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let user = extract_user_from_jwt(parts, &app_state.db, &app_state.auth_public_key).await;

        Ok(AppSession {
            user: match user {
                Ok(user) => Some(user),
                Err(_) => None,
            },
        })
    }
}

async fn extract_user_from_jwt(
    parts: &mut Parts,
    db: &DatabaseConnection,
    auth_public_key: &Vec<u8>,
) -> Result<entities::user::Model, BoxError> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await?;

    let jwt_user = decode::<JwtUser>(
        bearer.token(),
        &DecodingKey::from_rsa_pem(auth_public_key).unwrap(),
        &Validation::new(Algorithm::RS256),
    )?;

    let user = entities::user::Entity::find_by_id(jwt_user.claims.sub)
        .one(db)
        .await
        .unwrap()
        .unwrap();

    Ok(user)
}
