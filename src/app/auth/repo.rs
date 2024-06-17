use async_nats::jetstream::stream::No;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    ModelTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

use crate::app::users::entities::user_challenge;

use super::entities;

pub async fn find_user_by_email_with_credentials<T: ConnectionTrait>(
    db: &T,
    email: &String,
) -> Result<Option<(entities::user::Model, Vec<entities::user_credential::Model>)>, DbErr> {
    match entities::user::Entity::find()
        .filter(entities::user::Column::Email.eq(email))
        .one(db)
        .await?
    {
        Some(user) => {
            let user_credentials = user
                .find_related(entities::user_credential::Entity)
                .all(db)
                .await?;

            Ok(Some((user, user_credentials)))
        }
        None => Ok(None),
    }
}

pub async fn create_user_challenge<T: ConnectionTrait>(
    db: &T,
    model: entities::user_challenge::ActiveModel,
) -> Result<entities::user_challenge::Model, DbErr> {
    let user_passkey = model.insert(db).await?;

    Ok(user_passkey)
}

pub async fn find_user_challengle<T: ConnectionTrait>(
    db: &T,
    id: &String,
) -> Result<Option<entities::user_challenge::Model>, DbErr> {
    Ok(entities::user_challenge::Entity::find_by_id(id)
        .lock_exclusive()
        .one(db)
        .await?)
}

pub async fn create_user<T: ConnectionTrait>(
    db: &T,
    model: entities::user::Model,
) -> Result<entities::user::Model, DbErr> {
    let user = model.into_active_model().insert(db).await?;

    Ok(user)
}

pub async fn create_user_credential<T: ConnectionTrait>(
    db: &T,
    model: entities::user_credential::Model,
) -> Result<entities::user_credential::Model, DbErr> {
    let user = model.into_active_model().insert(db).await.unwrap();

    Ok(user)
}

pub async fn find_user_credential<T: ConnectionTrait>(
    db: &T,
    id: &String,
) -> Result<entities::user_credential::Model, DbErr> {
    Ok(entities::user_credential::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("".to_string()))?)
}

pub async fn delete_user_challengle<T: ConnectionTrait>(
    db: &T,
    model: entities::user_challenge::Model,
) -> Result<(), DbErr> {
    model.delete(db).await?;

    Ok(())
}
