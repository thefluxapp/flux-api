pub use sea_orm_migration::prelude::*;

mod m20221013_075812_create_users;
mod m20221016_114137_create_messages;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221013_075812_create_users::Migration),
            Box::new(m20221016_114137_create_messages::Migration)
        ]
    }
}
