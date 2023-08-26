use axum::{extract::State, Json};

use crate::app::{
    auth::{
        controller::AuthController,
        data::complete::{RequestData, ResponseData},
        service::AuthService,
    },
    AppState,
};

impl AuthController {
    pub async fn complete(
        State(state): State<AppState>,
        Json(data): Json<RequestData>,
    ) -> Json<ResponseData> {
        let user = AuthService::complete(&state.db, &state.webauthn, data.id, data.reg).await;
        let token = AuthService::generate_token(user.id);

        Json((user, token).into())
    }
}
