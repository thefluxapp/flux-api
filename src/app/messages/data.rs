use sea_orm::prelude::Uuid;
use serde::Serialize;

use super::entities;

#[derive(Serialize)]
pub struct IndexData {
    pub messages: String,
}

#[derive(Serialize)]
pub struct CreateData {
    pub message: MessageData,
}

#[derive(Serialize)]
pub struct MessageData {
    id: Uuid,
    text: String,
    user_id: Uuid,
}

impl From<entities::message::Model> for MessageData {
    fn from(message: entities::message::Model) -> Self {
        MessageData {
            id: message.id,
            text: message.text,
            user_id: message.user_id,
        }
    }
}
