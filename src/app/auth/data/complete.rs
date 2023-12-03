use serde::{Deserialize, Serialize};
use validator::Validate;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

use super::{super::entities, user::User};

#[derive(Deserialize, Validate, Debug)]
pub struct RequestData {
    pub id: String,
    pub reg: RegisterPublicKeyCredential,
    #[validate(length(min = 3))]
    pub first_name: String,
    #[validate(length(min = 3))]
    pub last_name: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub user: User,
    pub token: String,
}

impl From<(entities::user::Model, String)> for ResponseData {
    fn from((user, token): (entities::user::Model, String)) -> Self {
        ResponseData {
            user: User {
                id: user.id,
                name: user.name(),
                image: user.image(),
            },
            token,
        }
    }
}
