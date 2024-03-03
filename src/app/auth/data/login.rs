use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::PublicKeyCredential;

use crate::app::auth::User;

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

impl From<(User, String)> for ResponseData {
    fn from((user, token): (User, String)) -> Self {
        ResponseData { user, token }
    }
}
