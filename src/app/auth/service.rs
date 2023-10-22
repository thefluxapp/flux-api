use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
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
    pub fn generate_token(sub: Uuid) -> String {
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
