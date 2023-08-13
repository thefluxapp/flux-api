use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseTransaction, Set};

use super::entities;

pub struct TasksRepo {}

impl TasksRepo {
    pub async fn create_for_stream<T: ConnectionTrait>(
        db: &T,
        active_task: entities::task::ActiveModel,
    ) {
        active_task.insert(db).await.unwrap();
    }

    pub async fn mark_task_as_processed(
        db: &DatabaseTransaction,
        mut task: entities::task::ActiveModel,
    ) {
        task.processed_at = Set(Some(Utc::now().naive_utc()));
        task.update(db).await.unwrap();
    }
}
