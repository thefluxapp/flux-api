use chrono::Utc;
use migration::OnConflict;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{ConnectionTrait, Set};
use uuid::Uuid;

use super::entities;

pub struct StreamsRepo {}

impl StreamsRepo {
    pub async fn create_task<T: ConnectionTrait>(db: &T, stream: entities::stream::Model) {
        let stream_task = entities::stream_task::ActiveModel {
            stream_id: Set(stream.id),
            id: Set(Uuid::now_v7()),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        entities::stream_task::Entity::insert(stream_task)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .do_nothing()
            .exec(db)
            .await
            .unwrap();
    }

    pub async fn find_all_user_streams<T: ConnectionTrait>(
        db: &T,
    ) -> Vec<(entities::stream::Model, Option<entities::user::Model>)> {
        entities::stream::Entity::find()
            .find_also_related(entities::user::Entity)
            .filter(entities::stream::Column::MessageId.is_null())
            .all(db)
            .await
            .unwrap()
    }
}
