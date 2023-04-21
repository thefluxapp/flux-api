use axum::Json;

use super::super::data::index::ResponseData;
use crate::app::messages::services::MessagesServices;

use super::MessagesControllers;

impl MessagesControllers {
    pub async fn index() -> Json<ResponseData> {
        let messages = MessagesServices::index().await;

        Json(messages.into())
    }
}
