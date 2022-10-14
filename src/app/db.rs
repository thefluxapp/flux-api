use sea_orm::{Database, DatabaseConnection, ConnectOptions};

pub async fn create_pool(url: &String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(url.to_owned());

    opt.max_connections(15)
        .min_connections(5)
        .sqlx_logging(true);
        // .sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(opt).await.unwrap()
}
