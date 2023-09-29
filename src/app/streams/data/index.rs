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
    pub title: String,
    pub label: String,
    pub user: Option<ResponseStreamUserData>,
}

#[derive(Serialize)]
pub struct ResponseStreamUserData {
    pub id: Uuid,
    pub name: String,
}

impl From<Vec<(entities::stream::Model, Option<entities::user::Model>)>> for ResponseData {
    fn from(streams: Vec<(entities::stream::Model, Option<entities::user::Model>)>) -> Self {
        ResponseData {
            streams: streams.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<(entities::stream::Model, Option<entities::user::Model>)> for ResponseStreamData {
    fn from((stream, user): (entities::stream::Model, Option<entities::user::Model>)) -> Self {
        ResponseStreamData {
            id: stream.id,
            label: "#".to_string(),
            title: match stream.title {
                Some(title) => title,
                _ => match &user {
                    Some(user) => "Stream: ".to_string() + &user.name(),
                    _ => "VOID STREAM".to_string(),
                },
            },
            text: stream.text,
            user: match user {
                Some(user) => Some(ResponseStreamUserData {
                    id: user.id,
                    name: user.name(),
                }),
                _ => None,
            },
        }
    }
}
