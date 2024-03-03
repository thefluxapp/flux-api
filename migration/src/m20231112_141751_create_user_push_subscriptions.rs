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
                    .table(UserPushSubscriptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPushSubscriptions::Id)
                            .uuid()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserPushSubscriptions::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_push_subscriptions_user_id")
                            .from(UserPushSubscriptions::Table, UserPushSubscriptions::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(
                        ColumnDef::new(UserPushSubscriptions::Endpoint)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPushSubscriptions::P256dhKey)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPushSubscriptions::AuthKey)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPushSubscriptions::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPushSubscriptions::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum UserPushSubscriptions {
    Table,
    Id,
    UserId,
    Endpoint,
    P256dhKey,
    AuthKey,
    CreatedAt,
}
