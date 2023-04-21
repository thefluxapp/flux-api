use serde::Serialize;
use uuid::Uuid;

use super::{super::entities, index::ResponseStreamData};

#[derive(Serialize)]
pub struct ResponseData {
    stream: ResponseStreamData,
    messages: Vec<ResponseMessageData>,
}

#[derive(Serialize)]
pub struct ResponseMessageData {
    id: Uuid,
    text: String,
    stream: Option<ResponseStreamData>,
}

// #[derive(Serialize)]
// pub struct ResponseStreamData {
//     id: Uuid,
//     text: Option<String>,
// }

impl
    From<(
        entities::stream::Model,
        Vec<(entities::message::Model, Option<entities::stream::Model>)>,
    )> for ResponseData
{
    fn from(
        (stream, messages): (
            entities::stream::Model,
            Vec<(entities::message::Model, Option<entities::stream::Model>)>,
        ),
    ) -> Self {
        ResponseData {
            stream: stream.into(),
            messages: messages.into_iter().map(|x| x.into()).collect(),
        }
    }
}

// TODO: Try to refactor this
impl From<(entities::message::Model, Option<entities::stream::Model>)> for ResponseMessageData {
    fn from(
        (message, stream): (entities::message::Model, Option<entities::stream::Model>),
    ) -> Self {
        ResponseMessageData {
            id: message.id,
            text: message.text,
            stream: match stream {
                Some(stream) => Some(ResponseStreamData {
                    id: stream.id,
                    text: Some(String::from("QQQ")),
                }),
                None => None,
            },
        }
    }
}
