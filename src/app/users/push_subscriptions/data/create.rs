use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::users::push_subscriptions::entities;

#[derive(Serialize)]
pub struct ResponseData {
    pub id: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct RequestData {
    pub endpoint: String,
    pub p256dh_key: String,
    pub auth_key: String,
}

impl From<entities::push_subscription::Model> for ResponseData {
    fn from(push_subscription: entities::push_subscription::Model) -> Self {
        ResponseData {
            id: push_subscription.id,
        }
    }
}
