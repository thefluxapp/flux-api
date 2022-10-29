use fake::Fake;
use sea_orm::prelude::Uuid;
use serde::Serialize;

use super::entities;

#[derive(Serialize)]
pub struct StreamsIndexData {
    pub streams: Vec<StreamData>,
}

#[derive(Serialize)]
pub struct StreamData {
    id: Uuid,
    text: Option<String>,
}

impl From<Vec<entities::stream::Model>> for StreamsIndexData {
    fn from(streams: Vec<entities::stream::Model>) -> Self {
        StreamsIndexData {
            streams: streams.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<entities::stream::Model> for StreamData {
    fn from(stream: entities::stream::Model) -> Self {
        StreamData {
            id: stream.id,
            text: Some(fake::faker::lorem::raw::Sentence(fake::locales::EN, 3..6).fake()),
        }
    }
}

#[derive(Serialize)]
pub struct StreamsShowData {
    stream: StreamData,
    messages: Vec<MessageData>,
}

#[derive(Serialize)]
pub struct MessageData {
    id: Uuid,
    text: String,
    stream: Option<StreamData>,
}

impl
    From<(
        entities::stream::Model,
        Vec<(entities::message::Model, Option<entities::stream::Model>)>,
    )> for StreamsShowData
{
    fn from(
        (stream, messages): (
            entities::stream::Model,
            Vec<(entities::message::Model, Option<entities::stream::Model>)>,
        ),
    ) -> Self {
        StreamsShowData {
            stream: stream.into(),
            messages: messages.into_iter().map(|x| x.into()).collect(),
        }
    }
}

// TODO: Try to refactor this
impl From<(entities::message::Model, Option<entities::stream::Model>)> for MessageData {
    fn from(
        (message, stream): (entities::message::Model, Option<entities::stream::Model>),
    ) -> Self {
        MessageData {
            id: message.id,
            text: message.text,
            stream: match stream {
                Some(stream) => Some(StreamData {
                    id: stream.id,
                    text: Some(String::from("QQQ")),
                }),
                None => None,
            },
        }
    }
}
