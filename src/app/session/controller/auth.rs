use axum::{Extension, Json};

use crate::app::{
    session::{data::AuthData, service::SessionService},
    state::AppState,
};
use std::sync::Arc;

use super::SessionController;

impl SessionController {
    pub async fn auth(state: Extension<Arc<AppState>>) -> Json<AuthData> {
        // TODO: Return error if user exists
        let (user, token) = SessionService::auth(&state.db).await;
        Json(AuthData { id: user.id, token })
    }
}
