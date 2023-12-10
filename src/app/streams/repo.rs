use chrono::Utc;
use migration::OnConflict;
use sea_orm::QueryOrder;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use super::entities;

pub struct StreamsRepo {}

impl StreamsRepo {
    // pub async fn find_stream_by_id<T: ConnectionTrait>(
    //     db: &T,
    //     id: Uuid,
    // ) -> Option<entities::stream::Model> {
    //     entities::stream::Entity::find()
    //         .filter(entities::stream::Column::Id.eq(id))
    //         .one(db)
    //         .await
    //         .unwrap()
    // }

    pub async fn find_by_message_id<T: ConnectionTrait>(
        db: &T,
        message_id: Uuid,
    ) -> Option<entities::stream::Model> {
        entities::stream::Entity::find()
            .filter(entities::stream::Column::MessageId.eq(message_id))
            .one(db)
            .await
            .unwrap()
    }

    pub async fn find_stream_by_message_stream_id<T: ConnectionTrait>(
        db: &T,
        message_id: Uuid,
    ) -> Option<entities::stream::Model> {
        entities::stream::Entity::find()
            .inner_join(entities::message_stream::Entity)
            .filter(entities::message_stream::Column::MessageId.eq(message_id))
            .one(db)
            .await
            .unwrap()
    }

    pub async fn create<T: ConnectionTrait>(
        db: &T,
        message_id: Uuid,
        is_main: bool,
        title: Option<String>,
    ) -> entities::stream::Model {
        entities::stream::ActiveModel {
            title: Set(title),
            message_id: Set(message_id),
            is_main: Set(is_main),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap()
    }

    pub async fn create_stream_task<T: ConnectionTrait>(db: &T, stream_id: Uuid) {
        let stream_task = entities::stream_task::ActiveModel {
            stream_id: Set(stream_id),
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

    pub async fn find_streams<T: ConnectionTrait>(
        db: &T,
        is_main: bool,
    ) -> Vec<(entities::stream::Model, Option<entities::user::Model>)> {
        entities::stream::Entity::find()
            .find_also_related(entities::user::Entity)
            .filter(entities::stream::Column::IsMain.eq(is_main))
            .order_by_desc(entities::stream::Column::Id)
            .all(db)
            .await
            .unwrap()
    }
}
