use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use fake::Fake;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use uuid::Uuid;

use crate::app::{streams::entities, users::repo::UserRepo};

use super::SessionServices;

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub sub: Uuid,
    pub exp: u128,
}

impl SessionServices {
    pub async fn auth(db: &DatabaseConnection) -> (entities::user::Model, String) {
        let username = fake::faker::internet::raw::Username(fake::locales::EN).fake();
        let user = UserRepo::create_user(username, db).await;
        let token = SessionServices::generate_token(user.id);

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

#[cfg(test)]
mod tests {
    #[test]
    fn routing_to_auth() {}
}
