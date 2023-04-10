use std::time::Duration;

// use migration::{LockBehavior, LockType};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

use super::entities;

pub async fn run(pool: Arc<DatabaseConnection>) {
    let schd = JobScheduler::new().await.unwrap();

    schd.add(
        Job::new_repeated_async(Duration::from_secs(3), move |uuid, _l| {
            let x = pool.clone();

            Box::pin(async move {
                info!("I run every second: uuid={}", uuid);
                process_batch(&x).await;
            })
        })
        .unwrap(),
    )
    .await
    .unwrap();

    schd.start().await.unwrap();
}

async fn process_batch(pool: &DatabaseConnection) {
    let tasks = entities::task::Entity::find()
        .lock_exclusive()
        .limit(5)
        // .query()
        // .lock_with_behavior(LockType::Update, LockBehavior::Nowait)
        .all(pool)
        .await
        .unwrap();

    info!("tasks={:#?}", tasks);
}
