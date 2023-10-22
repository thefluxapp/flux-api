use sea_orm_migration::prelude::*;

use crate::{m20221016_114137_create_messages::Messages, m20221022_204621_create_streams::Streams};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Streams::Table)
                    .add_column(ColumnDef::new(Streams::MessageId).uuid())
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("fk_streams_message_id")
                            .from_tbl(Streams::Table)
                            .from_col(Streams::MessageId)
                            .to_tbl(Messages::Table)
                            .to_col(Messages::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Streams::Table)
                    .drop_foreign_key(Alias::new("fk_streams_message_id"))
                    .drop_column(Alias::new("message_id"))
                    .to_owned(),
            )
            .await
    }
}
