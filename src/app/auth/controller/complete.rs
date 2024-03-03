use axum::{extract::State, Json};
use validator::Validate;

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
        Json(request_data): Json<RequestData>,
    ) -> Json<ResponseData> {
        request_data.validate().unwrap();

        let user = AuthService::complete(
            &state.db,
            &state.webauthn,
            request_data.id,
            request_data.reg,
            request_data.first_name,
            request_data.last_name,
        )
        .await;
        let token = AuthService::generate_token(user.id).await;

        Json((user.into(), token).into())
    }
}
