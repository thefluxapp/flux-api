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
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tasks::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tasks::StreamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tasks-stream-id")
                            .from(Tasks::Table, Tasks::StreamId)
                            .to(Streams::Table, Streams::Id),
                    )
                    .col(ColumnDef::new(Tasks::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Tasks::ProcessedAt).date_time())
                    .col(ColumnDef::new(Tasks::FailedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Tasks {
    Table,
    Id,
    StreamId,
    CreatedAt,
    ProcessedAt,
    FailedAt,
}
