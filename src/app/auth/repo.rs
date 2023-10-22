use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait};

use super::entities;

pub struct AuthRepo {}

impl AuthRepo {
    pub async fn create_auth_state<T: ConnectionTrait>(
        db: &T,
        auth_state: entities::auth_state::ActiveModel,
    ) -> entities::auth_state::Model {
        auth_state.insert(db).await.unwrap()
    }

    pub async fn find_auth_state_by_id<T: ConnectionTrait>(
        db: &T,
        id: &String,
    ) -> entities::auth_state::Model {
        entities::auth_state::Entity::find_by_id(id)
            .one(db)
            .await
            .unwrap()
            .unwrap()
    }
}
