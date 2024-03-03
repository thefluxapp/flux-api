use sea_orm::{ActiveModelTrait, ConnectionTrait};

use super::entities;

pub struct PushSubscriptionsRepo {}

impl PushSubscriptionsRepo {
    pub async fn create_push_subscription<T: ConnectionTrait>(
        db: &T,
        push_subscription: entities::push_subscription::ActiveModel,
    ) -> entities::push_subscription::Model {
        push_subscription.insert(db).await.unwrap()
    }

    // pub async fn find_all<T: ConnectionTrait>(db: &T) -> Vec<entities::push_subscription::Model> {
    //     entities::push_subscription::Entity::find()
    //         .all(db)
    //         .await
    //         .unwrap()
    // }
}
