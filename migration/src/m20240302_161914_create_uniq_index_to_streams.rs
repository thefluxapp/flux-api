use sea_orm_migration::prelude::*;

use crate::m20221022_204621_create_streams::Streams;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_streams_message_id")
                    .unique()
                    .table(Streams::Table)
                    .col(Streams::MessageId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_streams_user_id")
                    .unique()
                    .table(Streams::Table)
                    .col(Streams::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_streams_message_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_streams_user_id").to_owned())
            .await?;

        Ok(())
    }
}
