use async_nats::Client;
use axum::BoxError;
use sea_orm::DbConn;
use serde::Serialize;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

mod entities;
mod repo;

pub struct Notifier {
    client: Client,
    db: Arc<DbConn>,
}

#[derive(Serialize)]
struct PushNotification {
    title: String,
    body: String,
    endpoint: String,
    p256dh_key: String,
    auth_key: String,
}

impl Notifier {
    pub async fn new(url: String, db: Arc<DbConn>) -> Self {
        Self {
            client: async_nats::connect(url).await.unwrap(),
            db,
        }
    }

    pub async fn send_push_notifications(&self, message_id: Uuid, text: String) {
        let push_subscriptions =
            repo::find_push_notifications_by_message_id(self.db.as_ref(), message_id)
                .await
                .unwrap();

        for push_subscription in push_subscriptions {
            self.send(
                String::from(""),
                text.clone(),
                push_subscription.endpoint,
                push_subscription.p256dh_key,
                push_subscription.auth_key,
            )
            .await
            .unwrap();
        }
    }

    pub async fn send(
        &self,
        title: String,
        body: String,
        endpoint: String,
        p256dh_key: String,
        auth_key: String,
    ) -> Result<(), BoxError> {
        let payload = serde_json::to_string(&PushNotification {
            title,
            body,
            endpoint,
            p256dh_key,
            auth_key,
        })?;

        self.client
            .publish("push-notifications", payload.into())
            .await?;

        info!("SEND EVENT");

        Ok(())
    }
}
