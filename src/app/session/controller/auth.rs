use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::app::{session::{service::SessionService, data::AuthData}, User};

use super::SessionController;

impl SessionController {
    pub async fn auth (_user: User, Extension(pool): Extension<DatabaseConnection>) -> Json<AuthData> {
        // TODO: Return error if user exists
        let (user, token) = SessionService::auth(&pool).await;

        Json(
            AuthData { id: user.id, token }
        )
    }
}
