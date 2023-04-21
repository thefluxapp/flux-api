use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use tracing::info;
use validator::Validate;

use crate::app::streams::services::StreamsServices;

use super::super::{data::create::RequestData, entities};
use super::MessagesServices;

impl MessagesServices {
    pub async fn create(
        user: &entities::user::Model,
        db: &DatabaseConnection,
        request_data: RequestData,
    ) -> entities::message::Model {
        request_data.validate().unwrap();

        let stream = if let Some(message_id) = request_data.message_id {
            let message = entities::message::Entity::find_by_id(message_id)
                .one(db)
                .await
                .unwrap()
                .unwrap();

            StreamsServices::find_or_create_by_message(message, db).await
        } else if let Some(stream_id) = request_data.stream_id {
            entities::stream::Entity::find_by_id(stream_id)
                .one(db)
                .await
                .unwrap()
                .unwrap()
        } else {
            StreamsServices::find_or_create_by_user(user, db).await
        };

        MessagesServices::create_with_stream(user, db, request_data, &stream).await
    }

    async fn create_with_stream(
        user: &entities::user::Model,
        db: &DatabaseConnection,
        request_data: RequestData,
        stream: &entities::stream::Model,
    ) -> entities::message::Model {
        let txn = db.begin().await.unwrap();

        let message = entities::message::ActiveModel {
            text: Set(request_data.text),
            user_id: Set(user.id),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .unwrap();

        entities::message_stream::ActiveModel {
            message_id: Set(message.id),
            stream_id: Set(stream.id),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .unwrap();

        let task = entities::task::ActiveModel {
            stream_id: Set(stream.id),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .unwrap();

        txn.commit().await.unwrap();

        info!("created task={:?}", task);
        info!("created message={:?}", &message);
        message
    }
}
