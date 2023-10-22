use sea_orm_migration::prelude::*;

use crate::{m20221016_114137_create_messages::Messages, m20221022_204621_create_streams::Streams};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MessagesStreams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MessagesStreams::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MessagesStreams::MessageId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_streams_message_id")
                            .from(MessagesStreams::Table, MessagesStreams::MessageId)
                            .to(Messages::Table, Messages::Id),
                    )
                    .col(ColumnDef::new(MessagesStreams::StreamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_streams_stream_id")
                            .from(MessagesStreams::Table, MessagesStreams::StreamId)
                            .to(Streams::Table, Streams::Id),
                    )
                    .col(
                        ColumnDef::new(MessagesStreams::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessagesStreams::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum MessagesStreams {
    Table,
    Id,
    MessageId,
    StreamId,
    CreatedAt,
}
