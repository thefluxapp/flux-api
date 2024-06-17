use sea_orm::{ConnectOptions, Database, DbConn};
use tracing::log;

use crate::settings::DatabaseSettings;

pub async fn create_pool(settings: &DatabaseSettings) -> DbConn {
    let mut opt = ConnectOptions::new(settings.endpoint.clone());

    opt.max_connections(settings.max_connections)
        .min_connections(settings.min_connections)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    Database::connect(opt).await.unwrap()
}
