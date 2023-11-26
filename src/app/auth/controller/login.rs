use axum::{extract::State, Json};

use crate::app::{
    auth::{
        controller::AuthController,
        data::login::{RequestData, ResponseData},
        service::AuthService,
    },
    AppState,
};

impl AuthController {
    pub async fn login(
        State(state): State<AppState>,
        Json(data): Json<RequestData>,
    ) -> Json<ResponseData> {
        let user = AuthService::login(&state.db, &state.webauthn, &data.id, &data.auth).await;
        let token = AuthService::generate_token(user.id).await;

        Json((user, token).into())
    }
}
