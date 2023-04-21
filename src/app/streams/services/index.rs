use super::{super::entities, StreamsServices};
use sea_orm::{DatabaseConnection, EntityTrait};

impl StreamsServices {
    pub async fn index(db: &DatabaseConnection) -> Vec<entities::stream::Model> {
        entities::stream::Entity::find().all(db).await.unwrap()
    }
}
