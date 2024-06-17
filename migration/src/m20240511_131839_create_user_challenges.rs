use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserChallenges::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserChallenges::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserChallenges::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserChallenges::UserName).string())
                    .col(
                        ColumnDef::new(UserChallenges::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserChallenges::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserChallenges {
    Table,
    Id,
    UserId,
    UserName,
    CreatedAt,
}
