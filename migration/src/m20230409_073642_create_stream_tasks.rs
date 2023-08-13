use sea_orm_migration::prelude::*;

use crate::m20221022_204621_create_streams::Streams;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StreamTasks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StreamTasks::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StreamTasks::StreamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stream_tasks_stream_id")
                            .from(StreamTasks::Table, StreamTasks::StreamId)
                            .to(Streams::Table, Streams::Id),
                    )
                    .col(
                        ColumnDef::new(StreamTasks::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StreamTasks::StartedAt).date_time())
                    .col(ColumnDef::new(StreamTasks::StartedBy).uuid())
                    .col(ColumnDef::new(StreamTasks::ProcessedAt).date_time())
                    .col(ColumnDef::new(StreamTasks::FailedAt).date_time())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_stream_tasks_started_at")
                    .table(StreamTasks::Table)
                    .col(StreamTasks::StartedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_stream_tasks_stream_id_started_at")
                    .unique()
                    .nulls_not_distinct()
                    .table(StreamTasks::Table)
                    .col(StreamTasks::StreamId)
                    .col(StreamTasks::StartedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StreamTasks::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum StreamTasks {
    Table,
    Id,
    StreamId,
    CreatedAt,
    ProcessedAt,
    StartedAt,
    StartedBy,
    FailedAt,
}
