use serde::Serialize;

use crate::app::auth::User;

#[derive(Serialize)]
pub struct ResponseData {
    pub user: Option<User>,
}

impl From<Option<User>> for ResponseData {
    fn from(user: Option<User>) -> Self {
        ResponseData { user }
    }
}
