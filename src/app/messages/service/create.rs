use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set, TransactionTrait};
use validator::Validate;

use crate::app::streams::service::StreamsService;

use super::super::{data::create::RequestData, entities, repo::MessagesRepo};
use super::MessagesService;

impl MessagesService {
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

            StreamsService::find_or_create_by_message(message, db).await
        } else if let Some(stream_id) = request_data.stream_id {
            entities::stream::Entity::find_by_id(stream_id)
                .one(db)
                .await
                .unwrap()
                .unwrap()
        } else {
            StreamsService::find_or_create_by_user(user, db).await
        };

        let message = entities::message::ActiveModel {
            text: Set(request_data.text),
            user_id: Set(user.id),
            ..Default::default()
        };

        db.transaction::<_, _, DbErr>(|txn| {
            Box::pin(
                async move { Ok(MessagesRepo::create_with_stream(txn, message, stream).await) },
            )
        })
        .await
        .unwrap()
    }
}
