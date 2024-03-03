use axum::Json;

use crate::app::auth::User;

use super::{super::data::index::ResponseData, AuthController};

impl AuthController {
    pub async fn index(user: Option<User>) -> Json<ResponseData> {
        Json(user.into())
    }
}
