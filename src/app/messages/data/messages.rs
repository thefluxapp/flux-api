use itertools::izip;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::super::entities;

#[derive(Deserialize, Validate)]
pub struct RequestData {
    #[validate(range(min = 2, max = 20))]
    pub limit: Option<u8>,
    pub before: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ResponseData {
    messages: Vec<ResponseMessageData>,
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
    text: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseMessageUserData {
    pub id: Uuid,
    pub name: String,
    pub image: String,
}

impl
    From<(
        Vec<entities::message::Model>,
        Vec<Option<entities::user::Model>>,
        Vec<Option<entities::stream::Model>>,
    )> for ResponseData
{
    fn from(
        (messages, users, streams): (
            Vec<entities::message::Model>,
            Vec<Option<entities::user::Model>>,
            Vec<Option<entities::stream::Model>>,
        ),
    ) -> Self {
        ResponseData {
            messages: izip!(messages, users, streams)
                .into_iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

impl
    From<(
        entities::message::Model,
        Option<entities::user::Model>,
        Option<entities::stream::Model>,
    )> for ResponseMessageData
{
    fn from(
        (message, user, stream): (
            entities::message::Model,
            Option<entities::user::Model>,
            Option<entities::stream::Model>,
        ),
    ) -> Self {
        ResponseMessageData {
            id: message.id,
            text: message.text.clone(),
            status: message.status().clone(),
            stream: match stream {
                Some(stream) => Some(ResponseMessageStreamData {
                    id: stream.id,
                    text: stream.text,
                }),
                _ => None,
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
        }
    }
}
