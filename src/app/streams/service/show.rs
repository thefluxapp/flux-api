use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
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
            Vec<(entities::message::Model, Option<entities::stream::Model>)>,
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

        // let messages = entities::message::Entity::find()
        //     .inner_join(entities::message_stream::Entity)
        //     .filter(entities::message_stream::Column::StreamId.eq(stream.id))
        //     .all(pool)
        //     .await
        //     .unwrap();

        let messages = Self::find_messages_with_streams_by_stream(stream.id, db).await;

        Ok((stream, messages))
    }

    async fn find_messages_with_streams_by_stream(
        stream_id: Uuid,
        pool: &DatabaseConnection,
    ) -> Vec<(entities::message::Model, Option<entities::stream::Model>)> {
        entities::message::Entity::find()
            .inner_join(entities::message_stream::Entity)
            .find_also_related(entities::stream::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .all(pool)
            .await
            .unwrap()
    }
}
