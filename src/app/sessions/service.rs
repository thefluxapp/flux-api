use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::app::User;

use super::{
    data::{AuthData, AuthPayload, SessionData},
    entities::user,
};
use fake::Fake;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::{DatabaseConnection, EntityTrait, Set};

pub fn show(user: User, _pool: &DatabaseConnection) -> SessionData {
    SessionData { user }
}

pub async fn auth(pool: &DatabaseConnection) -> AuthData {
    let new_user = user::ActiveModel {
        username: Set(fake::faker::internet::raw::Username(fake::locales::EN).fake()),
        ..Default::default()
    };
    let res = user::Entity::insert(new_user).exec(pool).await.unwrap();
    let payload = AuthPayload {
        sub: res.last_insert_id,
        exp: (SystemTime::now() + Duration::new(60 * 60 * 24 * 365, 0))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    };
    let token = encode(
        &Header::new(Algorithm::RS256),
        &payload,
        &EncodingKey::from_rsa_pem(&env::var("AUTH_PRIVATE_KEY").unwrap().into_bytes()).unwrap(),
    )
    .unwrap();

    AuthData {
        id: res.last_insert_id,
        token,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
