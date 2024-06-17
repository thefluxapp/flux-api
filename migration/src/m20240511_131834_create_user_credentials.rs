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
                    .table(UserCredentials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserCredentials::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserCredentials::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_credentials_user_id")
                            .from(UserCredentials::Table, UserCredentials::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(
                        ColumnDef::new(UserCredentials::PublicKey)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCredentials::PublicKeyAlgorithm)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCredentials::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserCredentials::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserCredentials {
    Table,
    Id,
    UserId,
    PublicKey,
    PublicKeyAlgorithm,
    CreatedAt,
}
