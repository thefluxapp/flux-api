use axum::Json;

use super::super::data::index::ResponseData;
use crate::app::messages::service::MessagesService;

use super::MessagesController;

impl MessagesController {
    pub async fn index() -> Json<ResponseData> {
        let messages = MessagesService::index().await;

        Json(messages.into())
    }
}
