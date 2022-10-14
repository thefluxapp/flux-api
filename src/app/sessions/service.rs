use std::env;

use super::{
    data::{AuthData, AuthPayload, SessionData},
    entities::user,
};
use fake::Fake;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::{DatabaseConnection, EntityTrait, Set, prelude::Uuid};

pub fn show(_pool: &DatabaseConnection) -> SessionData {
    dbg!(&env::var("AUTH_PRIVATE_KEY").unwrap());

    SessionData {
        id: Uuid::new_v4(),
    }
}

pub async fn auth(pool: &DatabaseConnection) -> AuthData {
    let new_user = user::ActiveModel {
        username: Set(fake::faker::internet::raw::Username(fake::locales::EN).fake()),
        ..Default::default()
    };
    let res = user::Entity::insert(new_user).exec(pool).await.unwrap();
    let payload = AuthPayload {
        sub: res.last_insert_id,
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
