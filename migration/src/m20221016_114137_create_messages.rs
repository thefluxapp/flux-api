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
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("default uuid_generate_v4()".to_string())
                            ,
                    )
                    .col(ColumnDef::new(Messages::Text).string().not_null())
                    .col(ColumnDef::new(Messages::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-user-id")
                            .from(Messages::Table, Messages::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Messages {
    Table,
    Id,
    Text,
    UserId,
}
