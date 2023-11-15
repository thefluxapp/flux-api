use async_nats::Client;
use axum::BoxError;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
    RelationTrait,
};
use serde::Serialize;
use tracing::info;
use uuid::Uuid;

mod entities;

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

    // message: entities::message::Model, stream: entities::stream::Model
    pub async fn notify(
        &self,
        db: &DatabaseConnection,
        message: entities::message::Model,
        stream: entities::stream::Model,
        current_user_id: Uuid,
    ) {
        let push_subscriptions = entities::push_subscription::Entity::find()
            .filter(
                entities::push_subscription::Column::UserId.in_subquery(
                    sea_orm::QueryFilter::query(
                        &mut entities::message::Entity::find()
                            .select_only()
                            .distinct()
                            .column(entities::message::Column::UserId)
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                entities::message::Relation::MessageStream.def(),
                            )
                            .filter(
                                Condition::all()
                                    .add(entities::message_stream::Column::StreamId.eq(stream.id))
                                    .add(entities::message::Column::UserId.ne(current_user_id)),
                            ),
                    )
                    .to_owned(),
                ),
            )
            .all(db)
            .await
            .unwrap();

        for push_subscription in push_subscriptions {
            self.send(
                stream.title.clone().unwrap_or("XXX".to_string()),
                message.text.clone(),
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