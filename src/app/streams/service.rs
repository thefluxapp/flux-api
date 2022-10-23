use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use super::entities;
use crate::app::User;

pub struct StreamService {}

// TODO: ADD ON CONFLICT
impl StreamService {
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
