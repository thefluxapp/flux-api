use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::Response,
    routing::get,
    Extension, Router,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, str::FromStr};

use self::users::entities;

mod db;
mod messages;
mod session;
mod streams;
mod tasks;
mod users;

pub async fn run() {
    let pool = db::create_pool(&env::var("DATABASE_URL").unwrap()).await;

    // TODO: Deal with it later
    Migrator::up(&pool, None).await.unwrap();

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/healthz", get(|| async {}))
                .nest("/session", session::router())
                .nest("/messages", messages::router())
                .nest("/streams", streams::router()),
        )
        .layer(Extension(pool));

    let addr = SocketAddr::from_str(&env::var("APP_ADDR").unwrap()).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct JwtUser {
    pub sub: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}

impl From<entities::user::Model> for User {
    fn from(user: entities::user::Model) -> Self {
        User {
            id: user.id,
            username: user.username,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .unwrap();

        let jwt_user = decode::<JwtUser>(
            bearer.token(),
            &DecodingKey::from_rsa_pem(&env::var("AUTH_PUBLIC_KEY").unwrap().into_bytes()).unwrap(),
            &Validation::new(Algorithm::RS256),
        )
        .unwrap();

        let Extension(pool) = Extension::<DatabaseConnection>::from_request_parts(parts, state)
            .await
            .unwrap();

        let user: User = entities::user::Entity::find_by_id(jwt_user.claims.sub)
            .one(&pool)
            .await
            .unwrap()
            .unwrap()
            .into();

        Ok(user)
    }
}
