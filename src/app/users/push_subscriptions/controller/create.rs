use axum::{extract::State, Json};

use crate::app::{auth::User, AppError, AppState};

use super::{
    super::{
        data::create::{RequestData, ResponseData},
        service,
    },
    PushSubscriptionsController,
};

impl PushSubscriptionsController {
    pub async fn create(
        user: User,
        State(state): State<AppState>,
        Json(request_data): Json<RequestData>,
    ) -> Result<Json<ResponseData>, AppError> {
        Ok(Json(
            service::PushSubscriptionsService::create(&user, &state.db, request_data)
                .await
                .into(),
        ))
    }
}
