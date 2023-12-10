use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::{
    super::{super::streams::repo::StreamsRepo, entities, repo::MessagesRepo},
    MessagesService,
};

impl MessagesService {
    pub async fn show(
        db: &DatabaseConnection,
        message_id: Uuid,
    ) -> Option<(
        entities::message::Model,
        Option<entities::user::Model>,
        Option<entities::stream::Model>,
    )> {
        match MessagesRepo::find_message_by_id(db, message_id).await {
            Some((message, user)) => {
                let stream = StreamsRepo::find_stream_by_message_stream_id(db, message.id).await;

                Some((message, user, stream))
            }
            None => None,
        }
    }
}
