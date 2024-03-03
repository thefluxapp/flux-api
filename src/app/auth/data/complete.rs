use serde::{Deserialize, Serialize};
use validator::Validate;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

use crate::app::auth::User;

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

impl From<(User, String)> for ResponseData {
    fn from((user, token): (User, String)) -> Self {
        ResponseData { user, token }
    }
}
