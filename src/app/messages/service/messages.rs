use sea_orm::{DatabaseConnection, LoaderTrait};
use uuid::Uuid;

use super::{
    super::{
        super::streams::repo::StreamsRepo, data::messages::RequestData, entities,
        repo::MessagesRepo,
    },
    MessagesService,
};

impl MessagesService {
    pub async fn messages(
        db: &DatabaseConnection,
        message_id: Uuid,
        request_data: RequestData,
    ) -> (
        Vec<entities::message::Model>,
        Vec<Option<entities::user::Model>>,
        Vec<Option<entities::stream::Model>>,
    ) {
        match StreamsRepo::find_by_message_id(db, message_id).await {
            Some(stream) => {
                let messages = MessagesRepo::find_by_stream_id_with_cursor(
                    db,
                    stream.id,
                    request_data.before,
                    request_data.limit,
                )
                .await;

                let users = messages.load_one(entities::user::Entity, db).await.unwrap();
                let streams = messages
                    .load_one(entities::stream::Entity, db)
                    .await
                    .unwrap();

                (messages, users, streams)
            }
            None => (vec![], vec![], vec![]),
        }
    }
}
