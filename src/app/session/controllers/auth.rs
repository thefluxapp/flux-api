use axum::{Extension, Json};

use crate::app::state::AppState;
use std::sync::Arc;

use super::super::{data::auth::ResponseData, services::SessionServices};
use super::SessionControllers;

impl SessionControllers {
    pub async fn auth(state: Extension<Arc<AppState>>) -> Json<ResponseData> {
        // TODO: Return error if user exists
        let (user, token) = SessionServices::auth(&state.db).await;
        Json(ResponseData { id: user.id, token })
    }
}
