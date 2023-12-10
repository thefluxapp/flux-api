use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect,
    QueryTrait, RelationTrait, Set,
};
use uuid::Uuid;

use super::entities;

pub struct MessagesRepo {}

impl MessagesRepo {
    pub async fn create_message<T: ConnectionTrait>(
        db: &T,
        text: String,
        user_id: Uuid,
    ) -> entities::message::Model {
        entities::message::ActiveModel {
            text: Set(text),
            user_id: Set(user_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap()
    }

    pub async fn create_message_stream<T: ConnectionTrait>(
        db: &T,
        message_id: Uuid,
        stream_id: Uuid,
    ) -> entities::message_stream::Model {
        entities::message_stream::ActiveModel {
            message_id: Set(message_id),
            stream_id: Set(stream_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap()
    }

    pub async fn find_message_by_id<T: ConnectionTrait>(
        db: &T,
        id: Uuid,
    ) -> Option<(entities::message::Model, Option<entities::user::Model>)> {
        entities::message::Entity::find_by_id(id)
            .find_also_related(entities::user::Entity)
            .one(db)
            .await
            .unwrap()
    }

    pub async fn find_by_stream_id_with_cursor<T: ConnectionTrait>(
        db: &T,
        stream_id: Uuid,
        before: Option<Uuid>,
        limit: Option<u8>,
    ) -> Vec<entities::message::Model> {
        entities::message::Entity::find()
            .inner_join(entities::message_stream::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .apply_if(before, |query, v| {
                query.filter(entities::message::Column::Id.lt(v))
            })
            .cursor_by(entities::message::Column::Id)
            .last(limit.unwrap_or(10).into())
            .all(db)
            .await
            .unwrap()
    }

    // pub async fn create_with_stream<T: ConnectionTrait>(
    //     db: &T,
    //     message: entities::message::ActiveModel,
    //     stream: entities::stream::Model,
    // ) -> entities::message::Model {
    //     let message = message.insert(db).await.unwrap();

    //     entities::message_stream::ActiveModel {
    //         message_id: Set(message.id),
    //         stream_id: Set(stream.id),
    //         ..Default::default()
    //     }
    //     .insert(db)
    //     .await
    //     .unwrap();

    //     // StreamsRepo::create_task(db, stream).await;

    //     message
    // }

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
