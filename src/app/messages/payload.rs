use sea_orm::prelude::Uuid;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateMessagePayload {
    #[validate(length(min = 1))]
    pub text: String,
    pub message_id: Option<Uuid>,
    pub stream_id: Option<Uuid>,
}
