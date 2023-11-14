use std::env;

use axum::Json;

use super::{super::data::vapid::ResponseData, PushSubscriptionsController};

impl PushSubscriptionsController {
    pub async fn vapid() -> Json<ResponseData> {
        Json(ResponseData {
            public_key: env::var("VAPID_PUBLIC_KEY").unwrap(),
        })
    }
}
