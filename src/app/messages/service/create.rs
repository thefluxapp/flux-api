use sea_orm::{DatabaseConnection, TransactionTrait};
use validator::Validate;

use crate::app::streams::repo::StreamsRepo;

use super::super::{data::create::RequestData, entities, repo::MessagesRepo};
use super::MessagesService;

impl MessagesService {
    pub async fn create(
        user: &entities::user::Model,
        db: &DatabaseConnection,
        request_data: RequestData,
    ) -> (entities::message::Model, entities::stream::Model) {
        request_data.validate().unwrap();

        let txn = db.begin().await.unwrap();

        let message = MessagesRepo::create_message(&txn, request_data.text, user.id).await;

        let stream = match request_data.message_id {
            Some(message_id) => match StreamsRepo::find_by_message_id(&txn, message_id).await {
                Some(stream) => stream,
                None => {
                    let stream = StreamsRepo::create(&txn, message_id, false, None).await;
                    MessagesRepo::create_message_stream(&txn, message_id, stream.id).await;

                    stream
                }
            },
            None => StreamsRepo::create(&txn, message.id, true, request_data.title).await,
        };

        MessagesRepo::create_message_stream(&txn, message.id, stream.id).await;
        StreamsRepo::create_stream_task(&txn, stream.id).await;

        txn.commit().await.unwrap();

        (message, stream)
    }
}
