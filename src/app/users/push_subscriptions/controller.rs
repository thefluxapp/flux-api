use axum::extract::State;
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::app::AppState;

use super::repo::PushSubscriptionsRepo;

mod create;
mod vapid;

pub struct PushSubscriptionsController {}

impl PushSubscriptionsController {
    pub async fn test(State(state): State<AppState>) {
        let db: &DatabaseConnection = state.db.as_ref();

        let ps = PushSubscriptionsRepo::find_all(db).await;

        for p in ps {
            state
                .notifier
                .send(
                    "title".to_string(),
                    "body".to_string(),
                    p.endpoint,
                    p.p256dh_key,
                    p.auth_key,
                )
                .await
                .unwrap();
        }
    }
}

#[derive(Serialize)]
struct Payload {
    title: String,
    body: String,
}
