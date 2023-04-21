use sea_orm::DatabaseConnection;

// TODO: Move to mod.rs
pub struct AppState {
    pub db: DatabaseConnection,
}
