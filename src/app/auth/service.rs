use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
use tokio::fs;
use uuid::Uuid;

mod complete;
mod login;

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub sub: Uuid,
    pub exp: u128,
}

pub struct AuthService {}

impl AuthService {
    pub async fn generate_token(sub: Uuid) -> String {
        let payload = AuthPayload {
            sub,
            exp: (SystemTime::now() + Duration::new(60 * 60 * 24 * 365, 0))
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        };

        // TODO: Do not read from file every time
        let auth_private_key = env::var("AUTH_PRIVATE_KEY_FILE").unwrap();
        let auth_private_key = fs::read_to_string(auth_private_key).await.unwrap();

        encode(
            &Header::new(Algorithm::RS256),
            &payload,
            &EncodingKey::from_rsa_pem(&auth_private_key.into_bytes()).unwrap(),
        )
        .unwrap()
    }
}
