use itertools::izip;
use serde::Serialize;
use uuid::Uuid;

use super::super::entities;

#[derive(Serialize)]
pub struct ResponseData {
    stream: ResponseStreamData,
    messages: Vec<ResponseMessageData>,
}

#[derive(Serialize)]
pub struct ResponseStreamData {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct ResponseMessageData {
    id: Uuid,
    text: String,
    status: String,
    stream: Option<ResponseMessageStreamData>,
    user: Option<ResponseMessageUserData>,
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
    pub label: String,
}

impl
    From<(
        entities::stream::Model,
        Vec<entities::message::Model>,
        Vec<Option<entities::user::Model>>,
        Vec<Option<entities::stream::Model>>,
    )> for ResponseData
{
    fn from(
        (stream, messages, users, streams): (
            entities::stream::Model,
            Vec<entities::message::Model>,
            Vec<Option<entities::user::Model>>,
            Vec<Option<entities::stream::Model>>,
        ),
    ) -> Self {
        ResponseData {
            stream: stream.into(),
            messages: izip!(messages, users, streams)
                .into_iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

// TODO: Try to refactor this
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
            text: message.text,
            status: "saved".to_string(),
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
                    label: user.name().chars().take(1).last().unwrap().to_string(),
                }),
                _ => None,
            },
        }
    }
}

impl From<entities::stream::Model> for ResponseStreamData {
    fn from(stream: entities::stream::Model) -> Self {
        ResponseStreamData { id: stream.id }
    }
}
