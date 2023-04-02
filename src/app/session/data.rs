use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct AuthData {
    pub id: Uuid,
    pub token: String,
}


#[derive(Serialize)]
pub struct ShowData {
    pub id: Uuid,
    pub username: String,
}
