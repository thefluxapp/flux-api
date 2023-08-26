use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::PublicKeyCredential;

use super::{super::entities, user::User};

#[derive(Deserialize)]
pub struct RequestData {
    pub id: String,
    pub auth: PublicKeyCredential,
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
                first_name: user.first_name,
                last_name: user.last_name,
            },
            token,
        }
    }
}
