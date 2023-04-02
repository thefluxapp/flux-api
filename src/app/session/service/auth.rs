use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use fake::Fake;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Serialize;
use uuid::Uuid;

use crate::app::streams::entities;

use super::SessionService;

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub sub: Uuid,
    pub exp: u128,
}

impl SessionService {
    pub async fn auth(pool: &DatabaseConnection) -> (entities::user::Model, String) {
        let user = entities::user::ActiveModel {
            id: Set(Uuid::now_v7()),
            username: Set(fake::faker::internet::raw::Username(fake::locales::EN).fake()),
            ..Default::default()
        };
        let user: entities::user::Model = user.insert(pool).await.unwrap();
        let token = Self::generate_token(user.id);

        (user, token)
    }

    fn generate_token(sub: Uuid) -> String {
        let payload = AuthPayload {
            sub,
            exp: (SystemTime::now() + Duration::new(60 * 60 * 24 * 365, 0))
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        };

        encode(
            &Header::new(Algorithm::RS256),
            &payload,
            &EncodingKey::from_rsa_pem(&env::var("AUTH_PRIVATE_KEY").unwrap().into_bytes())
                .unwrap(),
        )
        .unwrap()
    }
}
