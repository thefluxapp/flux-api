use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

use super::entities;

pub struct UserRepo {}

impl UserRepo {
    pub async fn create_user(username: String, db: &DatabaseConnection) -> entities::user::Model {
        let user = entities::user::ActiveModel {
            id: Set(Uuid::now_v7()),
            username: Set(username),
            ..Default::default()
        };

        user.insert(db).await.unwrap()
    }
}
