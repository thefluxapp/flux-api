use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::messages::entities;

// pub mod create;
pub mod index;
pub mod messages;
pub mod show;

#[derive(Deserialize, Validate, Debug)]
pub struct CreateRequestData {
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub text: String,
    pub message_id: Option<Uuid>,
    // pub stream_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct CreateResponseData {
    id: Uuid,
    status: String,
    order: i64,
}

impl From<entities::message::Model> for CreateResponseData {
    fn from(message: entities::message::Model) -> Self {
        Self {
            id: message.id,
            status: message.status(),
            order: message.order(),
        }
    }
}
