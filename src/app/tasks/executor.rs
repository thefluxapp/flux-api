use migration::LockType;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QuerySelect, TransactionTrait};
use std::{sync::Arc, time::Duration};
use tokio::time;

use super::{entities, repo::TasksRepo};

pub struct TasksExecutor {}

impl TasksExecutor {
    pub fn process_tasks(db: &DatabaseConnection) {
        let dbx = db.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(std::time::Duration::from_millis(500));

            loop {
                interval.tick().await;

                TasksExecutor::process_tasks_batch(&dbx).await;
            }
        });
    }

    async fn process_tasks_batch(db: &DatabaseConnection) {
        let txn = db.begin().await.unwrap();

        let mut query = entities::task::Entity::find().limit(2);

        query
            .query()
            .and_where(entities::task::Column::ProcessedAt.is_null())
            .lock_with_behavior(LockType::Update, migration::LockBehavior::SkipLocked);

        let tasks = query.all(&txn).await.unwrap();

        for task in tasks {
            // emulate external service
            tokio::time::sleep(Duration::from_millis(400)).await;

            TasksRepo::mark_task_as_processed(&txn, task.into()).await;
        }

        txn.commit().await.unwrap();
    }
}

pub async fn run(db: Arc<DatabaseConnection>) {
    TasksExecutor::process_tasks(&db);
}
