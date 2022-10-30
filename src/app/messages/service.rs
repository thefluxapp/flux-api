use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use validator::Validate;

use super::{
    data::{CreateData, IndexData},
    entities,
    payload::CreateMessagePayload,
};
use crate::app::{streams::service::StreamsService, User};

pub struct MessagesService {}

impl MessagesService {
    pub fn index() -> IndexData {
        IndexData {
            messages: String::from("MESSAGES"),
        }
    }

    pub async fn create(
        user: &User,
        pool: &DatabaseConnection,
        payload: CreateMessagePayload,
    ) -> CreateData {
        payload.validate().unwrap();

        let stream = if let Some(message_id) = payload.message_id {
            let message = entities::message::Entity::find_by_id(message_id)
                .one(pool)
                .await
                .unwrap()
                .unwrap();

            StreamsService::find_or_create_by_message(message, pool).await
        } else if let Some(stream_id) = payload.stream_id {
            entities::stream::Entity::find_by_id(stream_id)
                .one(pool)
                .await
                .unwrap()
                .unwrap()
        } else {
            StreamsService::find_or_create_by_user(user, pool).await
        };

        let message = MessagesService::create_with_stream(user, pool, payload, &stream).await;

        CreateData {
            message: message.into(),
        }
    }

    async fn create_with_stream(
        user: &User,
        pool: &DatabaseConnection,
        payload: CreateMessagePayload,
        stream: &entities::stream::Model,
    ) -> entities::message::Model {
        let txn = pool.begin().await.unwrap();

        let message = entities::message::ActiveModel {
            text: Set(payload.text),
            user_id: Set(user.id),
            ..Default::default()
        };

        let message: entities::message::Model = message.insert(&txn).await.unwrap();

        entities::message_stream::ActiveModel {
            message_id: Set(message.id),
            stream_id: Set(stream.id),
            ..Default::default()
        }
        .save(&txn)
        .await
        .unwrap();

        txn.commit().await.unwrap();

        message
    }
}

#[cfg(test)]
mod tests {}
