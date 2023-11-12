use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseData {
    pub public_key: String,
}
