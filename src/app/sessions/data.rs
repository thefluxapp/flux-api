use sea_orm::prelude::Uuid;
use serde::Serialize;

use crate::app::User;

#[derive(Serialize)]
pub struct SessionData {
    pub user: User,
}

#[derive(Serialize)]
pub struct AuthData {
    pub id: Uuid,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub sub: Uuid,
    pub exp: u128,
}
