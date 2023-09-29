use sea_orm::DatabaseConnection;
use std::time::Duration;
use tokio::time::{self, MissedTickBehavior};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;
use tracing::info;

use crate::app::{summarizer::Summarizer, AppState};

use super::service::TasksService;

pub struct TasksExecutor {}

impl TasksExecutor {
    pub fn process_tasks(db: &DatabaseConnection) {
        let tasks_service = TasksService {
            db: db.clone(),
            summarizer: Summarizer::new(),
        };

        let mut inteval = time::interval(Duration::from_millis(1000));
        inteval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        let mut stream = IntervalStream::new(inteval);

        tokio::spawn(async move {
            while let Some(_ts) = stream.next().await {
                tasks_service.process_stream_tasks().await;
            }
        });
    }
}

pub async fn run(state: &AppState) {
    TasksExecutor::process_tasks(&state.db);

    info!("Tasks executor started");
}
