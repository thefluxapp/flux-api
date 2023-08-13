use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
    TransactionTrait,
};

use super::entities;

pub struct TasksService {}

impl TasksService {
    // pub async fn process_summarize_stream<T: ConnectionTrait>(db: &T, task: entities::task::Model) {
    //     TasksRepo::mark_as_processed(db, task.into()).await;
    // }

    pub async fn process_streams_tasks(db: &DatabaseConnection) {
        let txn = db.begin().await.unwrap();

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
            .exec_with_returning(db)
            .await
            .unwrap();

        // TODO: make it async
        for stream_task in stream_tasks {
            println!("{:?}", stream_task)
        }

        txn.commit().await.unwrap();
    }
}
