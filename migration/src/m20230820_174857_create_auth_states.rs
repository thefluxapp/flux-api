use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthStates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthStates::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuthStates::State).json_binary().not_null())
                    .col(ColumnDef::new(AuthStates::Email).string().not_null())
                    .col(ColumnDef::new(AuthStates::UserId).uuid().not_null())
                    .col(ColumnDef::new(AuthStates::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("idx_auth_states_id")
                    .table(AuthStates::Table)
                    .col(AuthStates::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthStates::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum AuthStates {
    Table,
    Id,
    Email,
    UserId,
    State,
    CreatedAt,
}
