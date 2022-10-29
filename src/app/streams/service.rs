use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use super::entities;
use crate::app::User;

pub struct StreamsService {}

// TODO: ADD ON CONFLICT
impl StreamsService {
    pub async fn index(pool: &DatabaseConnection) -> Vec<entities::stream::Model> {
        entities::stream::Entity::find().all(pool).await.unwrap()
    }

    pub async fn show(
        stream_id: Uuid,
        pool: &DatabaseConnection,
    ) -> (
        entities::stream::Model,
        Vec<(entities::message::Model, Option<entities::stream::Model>)>,
    ) {
        let stream = entities::stream::Entity::find_by_id(stream_id)
            .one(pool)
            .await
            .unwrap()
            .unwrap();

        // let messages = entities::message::Entity::find()
        //     .inner_join(entities::message_stream::Entity)
        //     .filter(entities::message_stream::Column::StreamId.eq(stream.id))
        //     .all(pool)
        //     .await
        //     .unwrap();

        let messages = StreamsService::find_messages_with_streams_by_stream(stream.id, pool).await;

        (stream, messages)
    }

    async fn find_messages_with_streams_by_stream(
        stream_id: Uuid,
        pool: &DatabaseConnection,
    ) -> Vec<(entities::message::Model, Option<entities::stream::Model>)> {
        entities::message::Entity::find()
            .inner_join(entities::message_stream::Entity)
            .find_also_related(entities::stream::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .all(pool)
            .await
            .unwrap()
    }

    pub async fn find_or_create_by_user(
        user: &User,
        pool: &DatabaseConnection,
    ) -> entities::stream::Model {
        match entities::stream::Entity::find()
            .filter(entities::stream::Column::UserId.eq(user.id))
            .one(pool)
            .await
            .unwrap()
        {
            Some(stream) => stream,
            None => {
                let stream = entities::stream::ActiveModel {
                    user_id: Set(Some(user.id)),
                    ..Default::default()
                };

                stream.insert(pool).await.unwrap()
            }
        }
    }

    pub async fn find_or_create_by_message(
        message: entities::message::Model,
        pool: &DatabaseConnection,
    ) -> entities::stream::Model {
        match entities::stream::Entity::find()
            .filter(entities::stream::Column::MessageId.eq(message.id))
            .one(pool)
            .await
            .unwrap()
        {
            Some(stream) => stream,
            None => {
                let stream = entities::stream::ActiveModel {
                    message_id: Set(Some(message.id)),
                    ..Default::default()
                };

                stream.insert(pool).await.unwrap()
            }
        }
    }
}
