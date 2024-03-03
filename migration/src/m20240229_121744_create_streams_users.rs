use sea_orm_migration::prelude::*;

use crate::{m20221013_075812_create_users::Users, m20221022_204621_create_streams::Streams};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StreamsUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StreamsUsers::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StreamsUsers::StreamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_streams_users_stream_id")
                            .from(StreamsUsers::Table, StreamsUsers::StreamId)
                            .to(Streams::Table, Streams::Id),
                    )
                    .col(ColumnDef::new(StreamsUsers::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_streams_users_user_id")
                            .from(StreamsUsers::Table, StreamsUsers::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(
                        ColumnDef::new(StreamsUsers::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_streams_users_stream_id_user_id")
                    .unique()
                    .table(StreamsUsers::Table)
                    .col(StreamsUsers::StreamId)
                    .col(StreamsUsers::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StreamsUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StreamsUsers {
    Table,
    Id,
    StreamId,
    UserId,
    CreatedAt,
}
