use axum::{extract::State, Json};

use crate::app::{AppError, AppSession, AppState};

use super::{
    super::{
        data::create::{RequestData, ResponseData},
        service,
    },
    PushSubscriptionsController,
};

impl PushSubscriptionsController {
    pub async fn create(
        session: AppSession,
        State(state): State<AppState>,
        Json(request_data): Json<RequestData>,
    ) -> Result<Json<ResponseData>, AppError> {
        println!("{:?}", request_data);

        match session.user {
            Some(user) => Ok(Json(
                service::PushSubscriptionsService::create(&user, &state.db, request_data)
                    .await
                    .into(),
            )),
            None => Err(AppError::Forbidden),
        }
        // Json(ResponseData {
        //     id: "QQQ".to_string(),
        // })
    }
}
