use serde::Serialize;

use super::{super::entities, user::User};

#[derive(Serialize)]
pub struct ResponseData {
    pub user: Option<User>,
}

impl From<Option<entities::user::Model>> for ResponseData {
    fn from(user: Option<entities::user::Model>) -> Self {
        ResponseData {
            user: match user {
                Some(user) => Some(User {
                    id: user.id,
                    first_name: user.first_name,
                    last_name: user.last_name,
                }),
                _ => None,
            },
        }
    }
}
