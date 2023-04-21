use axum::Json;

use super::super::{data::show::ResponseData, Session};
use super::SessionControllers;

impl SessionControllers {
    pub async fn show(session: Session) -> Json<ResponseData> {
        Json(session.into())
    }
}
