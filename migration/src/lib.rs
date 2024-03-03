pub use sea_orm_migration::prelude::*;

mod m20221013_075812_create_users;
mod m20221016_114137_create_messages;
mod m20221022_204621_create_streams;
mod m20221022_205123_create_messages_streams;
mod m20221023_090254_add_message_to_streams;
mod m20230409_073642_create_stream_tasks;
mod m20230820_174857_create_auth_states;
mod m20231112_141751_create_user_push_subscriptions;
mod m20231209_150708_add_is_main_to_streams;
mod m20231210_170001_add_ya_gpt_id_to_stream_tasks;
mod m20240229_121744_create_streams_users;
mod m20240229_143819_create_uniq_index_to_messages_streams;
mod m20240302_161914_create_uniq_index_to_streams;
mod m20240303_142403_add_user_id_idx_to_pushes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221013_075812_create_users::Migration),
            Box::new(m20221016_114137_create_messages::Migration),
            Box::new(m20221022_204621_create_streams::Migration),
            Box::new(m20221022_205123_create_messages_streams::Migration),
            Box::new(m20221023_090254_add_message_to_streams::Migration),
            Box::new(m20230409_073642_create_stream_tasks::Migration),
            Box::new(m20230820_174857_create_auth_states::Migration),
            Box::new(m20231112_141751_create_user_push_subscriptions::Migration),
            Box::new(m20231209_150708_add_is_main_to_streams::Migration),
            Box::new(m20231210_170001_add_ya_gpt_id_to_stream_tasks::Migration),
            Box::new(m20240229_121744_create_streams_users::Migration),
            Box::new(m20240229_143819_create_uniq_index_to_messages_streams::Migration),
            Box::new(m20240302_161914_create_uniq_index_to_streams::Migration),
            Box::new(m20240303_142403_add_user_id_idx_to_pushes::Migration),
        ]
    }
}
