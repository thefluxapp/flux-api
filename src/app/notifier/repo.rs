use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect,
};
use uuid::Uuid;

use super::entities;

pub async fn find_push_notifications_by_message_id<T: ConnectionTrait>(
    db: &T,
    message_id: Uuid,
) -> Result<Vec<entities::push_subscription::Model>, DbErr> {
    let stream = entities::stream::Entity::find()
        .inner_join(entities::message_stream::Entity)
        .filter(entities::message_stream::Column::MessageId.eq(message_id))
        .one(db)
        .await?;

    match stream {
        Some(stream) => {
            let push_subscriptions = entities::push_subscription::Entity::find()
                .filter(entities::stream_user::Column::StreamId.eq(stream.id))
                .join(
                    JoinType::InnerJoin,
                    entities::push_subscription::Entity::belongs_to(entities::stream_user::Entity)
                        .from(entities::push_subscription::Column::UserId)
                        .to(entities::stream_user::Column::UserId)
                        .into(),
                )
                .all(db)
                .await?;

            Ok(push_subscriptions)
        }
        None => Ok(vec![]),
    }
}
