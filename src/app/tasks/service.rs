use chrono::Utc;
use itertools::Itertools;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use tracing::info;

use crate::app::messages::repo::MessagesRepo;

use super::super::{streams::repo::StreamsRepo, summarizer::ya_gpt::YaGPT};
use super::entities;

// TODO: make requests async
pub async fn process_stream_tasks(db: &DatabaseConnection, ya_gpt: &YaGPT) {
    let stream_tasks = StreamsRepo::find_and_lock_stream_tasks_batch(db).await;

    for stream_task in stream_tasks {
        info!("ST: {}", stream_task.id.to_string());

        match stream_task.ya_gpt_id.clone() {
            Some(ya_gpt_id) => get_ya_gpt_response(db, ya_gpt, stream_task, ya_gpt_id).await,
            None => create_ya_gpt_request(db, ya_gpt, stream_task).await,
        };
    }
}

// TODO: Refactor this
async fn create_ya_gpt_request(
    db: &DatabaseConnection,
    ya_gpt: &YaGPT,
    stream_task: entities::stream_task::Model,
) {
    let messages = MessagesRepo::get_by_stream(db, stream_task.stream_id).await;
    let text = messages
        .iter()
        .map(|(message, user)| {
            let name = match user {
                Some(user) => user.name(),
                _ => String::from("Person0"),
            };

            vec![name, message.text.clone()].join(": ")
        })
        .join("\n");

    let mut stream_task: entities::stream_task::ActiveModel = stream_task.into();
    stream_task.started_at = Set(None);

    match ya_gpt.ya_gpt_completion(text).await {
        Ok(ya_gpt_id) => {
            stream_task.ya_gpt_id = Set(Some(ya_gpt_id));
            stream_task.failed_at = Set(None);
        }
        Err(_) => {
            stream_task.failed_at = Set(Some(Utc::now().naive_utc()));
        }
    };

    stream_task.update(db).await.unwrap();
}

async fn get_ya_gpt_response(
    db: &DatabaseConnection,
    ya_gpt: &YaGPT,
    stream_task: entities::stream_task::Model,
    ya_gpt_id: String,
) {
    let stream_id = stream_task.stream_id;
    let mut stream_task: entities::stream_task::ActiveModel = stream_task.into();
    stream_task.started_at = Set(None);

    match ya_gpt.ya_gpt_operation(ya_gpt_id).await {
        Ok(text) => {
            let mut stream: entities::stream::ActiveModel = entities::stream::Entity::find()
                .filter(entities::stream::Column::Id.eq(stream_id))
                .one(db)
                .await
                .unwrap()
                .unwrap()
                .into();

            let txn = db.begin().await.unwrap();

            stream.text = Set(Some(text));
            stream.update(&txn).await.unwrap();

            let stream_task: entities::stream_task::ActiveModel = stream_task.into();
            stream_task.delete(&txn).await.unwrap();

            txn.commit().await.unwrap();
        }
        Err(err) => {
            dbg!(&err);

            stream_task.failed_at = Set(Some(Utc::now().naive_utc()));
            stream_task.update(db).await.unwrap();
        }
    };
}
