use sea_orm::DatabaseConnection;
use std::env;
use std::time::Duration;
use tokio::time::{self, MissedTickBehavior};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;

use crate::app::summarizer::ya_gpt::YaGPT;

use super::{super::AppState, service};

pub fn run(state: &AppState) {
    let db = state.db.clone();
    let ya_gpt = state.ya_gpt.clone();
    let period: u64 = env::var("STREAM_TASKS_PROCESSOR_PEROID_MS")
        .unwrap()
        .parse()
        .unwrap();

    tokio::spawn(async move {
        process_stream_tasks(&db, &ya_gpt, period).await;
    });
}

async fn process_stream_tasks(db: &DatabaseConnection, ya_gpt: &YaGPT, period: u64) {
    let mut interval = time::interval(Duration::from_millis(period));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut stream = IntervalStream::new(interval);

    while let Some(_ts) = stream.next().await {
        service::process_stream_tasks(db, ya_gpt).await;
    }
}
