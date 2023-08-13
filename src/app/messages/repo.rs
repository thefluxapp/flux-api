use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

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
}
