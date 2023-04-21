use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseData {
    pub messages: String,
}

impl From<String> for ResponseData {
    fn from(messages: String) -> Self {
        ResponseData { messages }
    }
}
