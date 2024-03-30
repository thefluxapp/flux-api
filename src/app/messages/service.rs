use sea_orm::{DatabaseConnection, DbConn, TransactionTrait};
use uuid::Uuid;

use crate::app::{auth::User, AppError};

use super::{data::CreateRequestData, entities, repo};

// mod create;
mod messages;
mod show;

pub struct MessagesService {}

pub async fn create_message(
    db: &DbConn,
    user: User,
    data: CreateRequestData,
) -> Result<entities::message::Model, AppError> {
    let (message, _) = match data.message_id {
        Some(message_id) => create_message_for_stream(db, data, user, message_id).await?,
        None => create_message_and_stream(db, data, user).await?,
    };

    Ok(message)
}

async fn create_message_for_stream(
    db: &DatabaseConnection,
    data: CreateRequestData,
    user: User,
    message_id: Uuid,
) -> Result<(entities::message::Model, entities::stream::Model), AppError> {
    let stream = match repo::find_steam_by_message_id(db, message_id).await? {
        Some(stream) => stream,
        None => {
            repo::create_stream_and_select_again(
                db,
                entities::stream::Model {
                    id: Uuid::now_v7(),
                    title: data.title,
                    text: None,
                    user_id: Some(user.id),
                    message_id,
                    is_main: false,
                    created_at: chrono::Utc::now().naive_utc(),
                },
            )
            .await?
        }
    };

    let txn = db.begin().await?;

    let message = repo::create_message(
        &txn,
        entities::message::Model {
            id: Uuid::now_v7(),
            text: data.text,
            user_id: user.id,
            created_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::create_message_stream(
        &txn,
        entities::message_stream::Model {
            id: Uuid::now_v7(),
            message_id: message.id,
            stream_id: stream.id,
            created_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::create_stream_task(
        &txn,
        entities::stream_task::Model {
            id: Uuid::now_v7(),
            stream_id: stream.id,
            created_at: chrono::Utc::now().naive_utc(),
            processed_at: None,
            started_at: None,
            started_by: None,
            ya_gpt_id: None,
            failed_at: None,
        },
    )
    .await?;

    repo::create_stream_user(
        &txn,
        entities::stream_user::Model {
            id: Uuid::now_v7(),
            stream_id: stream.id,
            created_at: chrono::Utc::now().naive_utc(),
            user_id: user.id,
        },
    )
    .await?;

    txn.commit().await?;

    Ok((message, stream))
}

async fn create_message_and_stream(
    db: &DatabaseConnection,
    data: CreateRequestData,
    user: User,
) -> Result<(entities::message::Model, entities::stream::Model), AppError> {
    let txn = db.begin().await?;

    let message = repo::create_message(
        &txn,
        entities::message::Model {
            id: Uuid::now_v7(),
            text: data.text,
            user_id: user.id,
            created_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    let stream = repo::create_stream(
        &txn,
        entities::stream::Model {
            id: Uuid::now_v7(),
            title: data.title,
            text: None,
            user_id: None,
            message_id: message.id,
            is_main: true,
            created_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::create_message_stream(
        &txn,
        entities::message_stream::Model {
            id: Uuid::now_v7(),
            message_id: message.id,
            stream_id: stream.id,
            created_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::create_stream_task(
        &txn,
        entities::stream_task::Model {
            id: Uuid::now_v7(),
            stream_id: stream.id,
            created_at: chrono::Utc::now().naive_utc(),
            processed_at: None,
            started_at: None,
            started_by: None,
            ya_gpt_id: None,
            failed_at: None,
        },
    )
    .await?;

    repo::create_stream_user(
        &txn,
        entities::stream_user::Model {
            id: Uuid::now_v7(),
            stream_id: stream.id,
            created_at: chrono::Utc::now().naive_utc(),
            user_id: user.id,
        },
    )
    .await?;

    txn.commit().await?;

    Ok((message, stream))
}
