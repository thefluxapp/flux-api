use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::app::AppError;

use super::{super::entities, StreamsService};

impl StreamsService {
    pub async fn show(
        db: &DatabaseConnection,
        stream_id: Uuid,
    ) -> Result<
        (
            entities::stream::Model,
            Vec<entities::message::Model>,
            Vec<Option<entities::user::Model>>,
            Vec<Option<entities::stream::Model>>,
        ),
        AppError,
    > {
        let stream = match entities::stream::Entity::find_by_id(stream_id)
            .one(db)
            .await
            .unwrap()
        {
            Some(stream) => stream,
            None => return Err(AppError::EntityNotFound),
        };

        let (messages, users, streams) =
            Self::find_messages_with_streams_by_stream(db, stream.id).await;

        Ok((stream, messages, users, streams))
    }

    async fn find_messages_with_streams_by_stream(
        db: &DatabaseConnection,
        stream_id: Uuid,
    ) -> (
        Vec<entities::message::Model>,
        Vec<Option<entities::user::Model>>,
        Vec<Option<entities::stream::Model>>,
    ) {
        let messages = entities::message::Entity::find()
            .inner_join(entities::message_stream::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .all(db)
            .await
            .unwrap();

        let users = messages.load_one(entities::user::Entity, db).await.unwrap();
        let streams = messages
            .load_one(entities::stream::Entity, db)
            .await
            .unwrap();

        (messages, users, streams)
    }
}
