use async_nats::Client;
use axum::BoxError;
use serde::Serialize;
use tracing::info;

pub struct Notifier {
    client: Client,
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
    pub async fn new(url: String) -> Self {
        Self {
            client: async_nats::connect(url).await.unwrap(),
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

        Ok(())
    }
}
