use axum::Json;

use crate::app::{session::data::ShowData, User};

use super::SessionController;

impl SessionController {
    pub async fn show(user: User) -> Json<ShowData> {
        Json(ShowData {
            id: user.id,
            username: user.username,
        })
    }
}
