use axum::Json;

use crate::app::AppSession;

use super::{super::data::index::ResponseData, AuthController};

impl AuthController {
    pub async fn index(session: AppSession) -> Json<ResponseData> {
        Json(session.user.into())
    }
}
