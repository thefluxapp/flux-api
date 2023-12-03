use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::messages::entities;

#[derive(Deserialize, Validate)]
pub struct RequestData {
    #[validate(length(min = 1))]
    pub text: String,
    pub message_id: Option<Uuid>,
    pub stream_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ResponseData {
    id: Uuid,
    order: i64,
}

impl From<entities::message::Model> for ResponseData {
    fn from(message: entities::message::Model) -> Self {
        ResponseData {
            id: message.id,
            order: message.created_at.timestamp_micros(),
        }
    }
}
