use sea_orm_migration::prelude::*;

use crate::{
    m20231112_141751_create_user_push_subscriptions::UserPushSubscriptions,
    m20240229_121744_create_streams_users::StreamsUsers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_streams_users_user_id")
                    .table(UserPushSubscriptions::Table)
                    .col(UserPushSubscriptions::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_push_subscriptions_user_id")
                    .table(StreamsUsers::Table)
                    .col(StreamsUsers::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_streams_users_user_id").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_push_subscriptions_user_id")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
