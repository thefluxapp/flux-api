use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::app::tasks::repo::TasksRepo;

use super::entities;

pub struct MessagesRepo {}

impl MessagesRepo {
    pub async fn create_with_stream<T: ConnectionTrait>(
        db: &T,
        active_message: entities::message::ActiveModel,
        stream: entities::stream::Model,
    ) -> entities::message::Model {
        let message = active_message.insert(db).await.unwrap();

        entities::message_stream::ActiveModel {
            message_id: Set(message.id),
            stream_id: Set(stream.id),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();

        let active_task = entities::task::ActiveModel {
            stream_id: Set(stream.id),
            ..Default::default()
        };

        TasksRepo::create_for_stream(db, active_task).await;

        message
    }
}
