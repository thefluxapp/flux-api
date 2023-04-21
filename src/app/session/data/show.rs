use serde::Serialize;
use uuid::Uuid;

use crate::app::session::Session;

#[derive(Serialize)]
pub struct ResponseData {
    user: Option<ResponseUserData>,
}

#[derive(Serialize)]
pub struct ResponseUserData {
    id: Uuid,
    username: String,
}

impl From<Session> for ResponseData {
    fn from(session: Session) -> Self {
        ResponseData {
            user: match session.user {
                Some(user) => Some(ResponseUserData {
                    id: user.id,
                    username: user.username,
                }),
                None => None,
            },
        }
    }
}
