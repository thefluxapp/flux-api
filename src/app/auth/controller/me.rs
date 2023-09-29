use axum::Json;

use crate::app::AppSession;

use super::{super::data::me::ResponseData, AuthController};

impl AuthController {
    pub async fn me(session: AppSession) -> Json<ResponseData> {
        Json(session.user.into())
    }
}
