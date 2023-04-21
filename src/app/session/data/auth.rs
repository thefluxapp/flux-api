use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ResponseData {
    pub id: Uuid,
    pub token: String,
}
