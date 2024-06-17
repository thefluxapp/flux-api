use sea_orm::{
    sea_query::OnConflict, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, QuerySelect, QueryTrait, RelationTrait,
};
use uuid::Uuid;

use super::entities;

pub struct MessagesRepo {}

impl MessagesRepo {
    pub async fn find_message_by_id<T: ConnectionTrait>(
        db: &T,
        id: Uuid,
    ) -> Option<(entities::message::Model, Option<entities::user::Model>)> {
        entities::message::Entity::find_by_id(id)
            .find_also_related(entities::user::Entity)
            .one(db)
            .await
            .unwrap()
    }

    pub async fn find_by_stream_id_with_cursor<T: ConnectionTrait>(
        db: &T,
        stream_id: Uuid,
        before: Option<Uuid>,
        limit: Option<u8>,
    ) -> Vec<entities::message::Model> {
        entities::message::Entity::find()
            .inner_join(entities::message_stream::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .apply_if(before, |query, v| {
                query.filter(entities::message::Column::Id.lt(v))
            })
            .cursor_by(entities::message::Column::Id)
            .last(limit.unwrap_or(10).into())
            .all(db)
            .await
            .unwrap()
    }

    pub async fn get_by_stream<T: ConnectionTrait>(
        db: &T,
        stream_id: Uuid,
    ) -> Vec<(
        entities::message::Model,
        std::option::Option<entities::user::Model>,
    )> {
        entities::message::Entity::find()
            .find_also_related(entities::user::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .join(
                sea_orm::JoinType::InnerJoin,
                entities::message::Relation::MessageStream.def(),
            )
            .all(db)
            .await
            .unwrap()
    }
}

pub async fn create_message<T: ConnectionTrait>(
    db: &T,
    model: entities::message::Model,
) -> Result<entities::message::Model, DbErr> {
    let message = model.into_active_model().insert(db).await?;

    Ok(message)
}

pub async fn create_stream<T: ConnectionTrait>(
    db: &T,
    model: entities::stream::Model,
) -> Result<entities::stream::Model, DbErr> {
    let stream = model.into_active_model().insert(db).await?;

    Ok(stream)
}

pub async fn create_stream_and_select_again<T: ConnectionTrait>(
    db: &T,
    model: entities::stream::Model,
) -> Result<entities::stream::Model, DbErr> {
    let message_id = model.message_id;

    entities::stream::Entity::insert(model.into_active_model())
        .on_conflict(
            OnConflict::column(entities::stream::Column::MessageId)
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(db)
        .await?;

    let stream = entities::stream::Entity::find()
        .filter(entities::stream::Column::MessageId.eq(message_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotInserted)?;

    entities::message_stream::Entity::insert(
        entities::message_stream::Model {
            id: Uuid::now_v7(),
            message_id: stream.message_id,
            stream_id: stream.id,
            created_at: chrono::Utc::now().naive_utc(),
        }
        .into_active_model(),
    )
    .on_conflict(OnConflict::new().do_nothing().to_owned())
    .do_nothing()
    .exec(db)
    .await?;

    Ok(stream)
}

pub async fn create_message_stream<T: ConnectionTrait>(
    db: &T,
    model: entities::message_stream::Model,
) -> Result<entities::message_stream::Model, DbErr> {
    let message_stream = model.into_active_model().insert(db).await?;

    Ok(message_stream)
}

pub async fn find_steam_by_message_id<T: ConnectionTrait>(
    db: &T,
    message_id: Uuid,
) -> Result<Option<entities::stream::Model>, DbErr> {
    let stream = entities::stream::Entity::find()
        .filter(entities::stream::Column::MessageId.eq(message_id))
        .one(db)
        .await?;

    Ok(stream)
}

pub async fn create_stream_task<T: ConnectionTrait>(
    db: &T,
    model: entities::stream_task::Model,
) -> Result<(), DbErr> {
    entities::stream_task::Entity::insert(model.into_active_model())
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .do_nothing()
        .exec(db)
        .await?;

    Ok(())
}

pub async fn create_stream_user<T: ConnectionTrait>(
    db: &T,
    model: entities::stream_user::Model,
) -> Result<(), DbErr> {
    entities::stream_user::Entity::insert(model.into_active_model())
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .do_nothing()
        .exec(db)
        .await?;

    Ok(())
}
