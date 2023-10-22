use chrono::Utc;
use itertools::Itertools;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect, Set, TransactionTrait,
};

// use crate::app::summarizer::Summarizer;

use crate::app::messages::repo::MessagesRepo;
use crate::app::summarizer::Summarizer;

use super::entities;

pub struct TasksService {
    pub db: DatabaseConnection,
    pub summarizer: Summarizer,
}

impl TasksService {
    pub async fn process_stream_tasks(&self) {
        let txn = self.db.begin().await.unwrap();

        let stream_tasks = entities::stream_task::Entity::update_many()
            .col_expr(
                entities::stream_task::Column::StartedAt,
                Expr::value(Utc::now().naive_utc()),
            )
            .filter(
                entities::stream_task::Column::Id.in_subquery(
                    sea_orm::QueryFilter::query(
                        &mut entities::stream_task::Entity::find()
                            .select_only()
                            .column(entities::stream_task::Column::Id)
                            .filter(
                                Condition::any()
                                    .add(entities::stream_task::Column::StartedAt.is_null()),
                            )
                            .limit(2),
                    )
                    .to_owned(),
                ),
            )
            .exec_with_returning(&txn)
            .await
            .unwrap();

        txn.commit().await.unwrap();

        for stream_task in stream_tasks {
            self.process_stream_task(stream_task).await;
        }
    }

    async fn process_stream_task(&self, stream_task: entities::stream_task::Model) {
        // let text = String::from("");

        let messages = MessagesRepo::get_by_stream(&self.db, stream_task.stream_id).await;
        let text = messages
            .iter()
            .map(|(message, user)| {
                vec![
                    match user {
                        Some(user) => user.name(),
                        _ => String::from("Nobody"),
                    },
                    message.text.clone(),
                ]
                .join(": ")
            })
            .join("\n");

        match self.summarizer.call(text).await {
            Ok(res) => {
                let txn = self.db.begin().await.unwrap();

                let mut stream: entities::stream::ActiveModel =
                    entities::stream::Entity::find_by_id(stream_task.stream_id)
                        .one(&txn)
                        .await
                        .unwrap()
                        .unwrap()
                        .into();

                stream.text = Set(Some(res.text));
                stream.update(&txn).await.unwrap();

                let stream_task: entities::stream_task::ActiveModel = stream_task.into();
                stream_task.delete(&txn).await.unwrap();

                txn.commit().await.unwrap();

                println!("GETTING SUCCESS FROM AI");
            }
            _ => {
                let mut stream_task: entities::stream_task::ActiveModel = stream_task.into();
                stream_task.started_at = Set(None);
                stream_task.failed_at = Set(Some(Utc::now().naive_utc()));
                stream_task.update(&self.db).await.unwrap();

                println!("REQUEST FAILED");
            }
        };
    }
}
