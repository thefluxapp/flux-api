use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect,
    RelationTrait, Set,
};
use uuid::Uuid;

use super::entities;
use crate::app::streams::repo::StreamsRepo;

pub struct MessagesRepo {}

impl MessagesRepo {
    pub async fn create_with_stream<T: ConnectionTrait>(
        db: &T,
        message: entities::message::ActiveModel,
        stream: entities::stream::Model,
    ) -> entities::message::Model {
        let message = message.insert(db).await.unwrap();

        entities::message_stream::ActiveModel {
            message_id: Set(message.id),
            stream_id: Set(stream.id),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();

        StreamsRepo::create_task(db, stream).await;

        message
    }

    pub async fn get_by_stream<T: ConnectionTrait>(
        db: &T,
        stream_id: Uuid,
    ) -> Vec<(
        entities::message::Model,
        std::option::Option<entities::user::Model>,
    )> {
        entities::message::Entity::find()
            .find_also_related(entities::user::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .join(
                sea_orm::JoinType::InnerJoin,
                entities::message::Relation::MessageStream.def(),
            )
            .all(db)
            .await
            .unwrap()
    }
}
