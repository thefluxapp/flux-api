use std::{env, error::Error};

use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::Response,
    Extension, RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

use super::{entities, JwtUser, Session};
use crate::app::state::AppState;

#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Extension::<Arc<AppState>>::from_request_parts(parts, state)
            .await
            .unwrap();

        let user = extract_user_from_jwt(parts, &state.db).await;

        Ok(Session {
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
) -> Result<entities::user::Model, Box<dyn Error>> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await?;

    let jwt_user = decode::<JwtUser>(
        bearer.token(),
        &DecodingKey::from_rsa_pem(&env::var("AUTH_PUBLIC_KEY").unwrap().into_bytes()).unwrap(),
        &Validation::new(Algorithm::RS256),
    )?;

    let user = entities::user::Entity::find_by_id(jwt_user.claims.sub)
        .one(db)
        .await?
        .expect("NO_USER_FOR_JWT_SUB");

    Ok(user)
}
