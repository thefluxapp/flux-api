use fake::Fake;
use serde::Serialize;
use uuid::Uuid;

use super::super::entities;

#[derive(Serialize)]
pub struct ResponseData {
    pub streams: Vec<ResponseStreamData>,
}

#[derive(Serialize)]
pub struct ResponseStreamData {
    pub id: Uuid,
    pub text: Option<String>,
}

impl From<Vec<entities::stream::Model>> for ResponseData {
    fn from(streams: Vec<entities::stream::Model>) -> Self {
        ResponseData {
            streams: streams.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<entities::stream::Model> for ResponseStreamData {
    fn from(stream: entities::stream::Model) -> Self {
        ResponseStreamData {
            id: stream.id,
            text: Some(fake::faker::lorem::raw::Sentence(fake::locales::EN, 3..6).fake()),
        }
    }
}
