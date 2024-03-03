use serde::Serialize;
use uuid::Uuid;

use super::super::entities;

#[derive(Serialize)]
pub struct ResponseData {
    message: ResponseMessageData,
}

#[derive(Serialize)]
pub struct ResponseMessageData {
    id: Uuid,
    text: String,
    status: String,
    stream: Option<ResponseMessageStreamData>,
    user: Option<ResponseMessageUserData>,
    order: i64,
}

#[derive(Serialize)]
pub struct ResponseMessageStreamData {
    id: Uuid,
    message_id: Uuid,
    text: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseMessageUserData {
    pub id: Uuid,
    pub name: String,
    pub image: Option<String>,
}

impl
    From<(
        entities::message::Model,
        Option<entities::user::Model>,
        Option<entities::stream::Model>,
    )> for ResponseData
{
    fn from(
        (message, user, stream): (
            entities::message::Model,
            Option<entities::user::Model>,
            Option<entities::stream::Model>,
        ),
    ) -> Self {
        ResponseData {
            message: ResponseMessageData {
                id: message.id,
                text: message.text.clone(),
                status: message.status().clone(),
                stream: match stream {
                    Some(stream) => Some(ResponseMessageStreamData {
                        id: stream.id,
                        message_id: stream.message_id,
                        text: stream.text,
                    }),
                    None => None,
                },
                user: match user {
                    Some(user) => Some(ResponseMessageUserData {
                        id: user.id,
                        name: user.name(),
                        image: user.image(),
                    }),
                    _ => None,
                },
                order: message.order(),
            },
        }
    }
}
