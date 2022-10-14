use sea_orm::prelude::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct SessionData {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct AuthData {
    pub id: Uuid,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub sub: Uuid
}
