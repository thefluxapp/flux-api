use sea_orm::DatabaseConnection;

use super::{super::entities, super::repo::StreamsRepo, StreamsService};

impl StreamsService {
    pub async fn index(
        db: &DatabaseConnection,
    ) -> Vec<(entities::stream::Model, Option<entities::user::Model>)> {
        StreamsRepo::find_streams(db, true).await
    }
}
