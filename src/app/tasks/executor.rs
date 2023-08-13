use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::time;
use tracing::info;

use crate::app::state::AppState;

use super::service::TasksService;

pub struct TasksExecutor {}

impl TasksExecutor {
    pub fn process_tasks(db: &DatabaseConnection) {
        let dbx = db.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(std::time::Duration::from_millis(5000));

            loop {
                interval.tick().await;

                TasksService::process_streams_tasks(&dbx).await;
            }
        });
    }
}

pub async fn run(state: &Arc<AppState>) {
    TasksExecutor::process_tasks(&state.db);
    info!("Tasks executor start");
}
