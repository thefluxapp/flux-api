pub use sea_orm_migration::prelude::*;

mod m20221013_075812_create_users;
mod m20221016_114137_create_messages;
mod m20221022_204621_create_streams;
mod m20221022_205123_create_messages_streams;
mod m20221023_090254_add_message_to_streams;
mod m20230409_073642_create_stream_tasks;
mod m20230820_174857_create_auth_states;

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
        ]
    }
}
