use sea_orm_migration::prelude::*;

use crate::m20221013_075812_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Streams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Streams::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("default uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(Streams::Title).string())
                    .col(ColumnDef::new(Streams::UserId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-streams-user-id")
                            .from(Streams::Table, Streams::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(
                        ColumnDef::new(Streams::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Streams::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Streams {
    Table,
    Id,
    Title,
    UserId,
    MessageId,
    CreatedAt,
}
