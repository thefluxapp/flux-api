use sea_orm_migration::prelude::*;

use crate::m20221022_205123_create_messages_streams::MessagesStreams;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_streams_message_id_stream_id")
                    .unique()
                    .table(MessagesStreams::Table)
                    .col(MessagesStreams::StreamId)
                    .col(MessagesStreams::MessageId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_messages_streams_message_id_stream_id")
                    .to_owned(),
            )
            .await
    }
}
