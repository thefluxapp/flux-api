use axum::Json;

use crate::app::{User, session::data::ShowData};

use super::SessionController;

impl SessionController {
    pub async fn show(user: User) -> Json<ShowData> {
        Json(
            ShowData {
                id: user.id,
                username: user.username
            }
        )
    }
}
