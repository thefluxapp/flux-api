use sea_orm::{DatabaseConnection, Set};

use super::PushSubscriptionsService;

use super::super::{data::create::RequestData, entities, repo::PushSubscriptionsRepo};

impl PushSubscriptionsService {
    pub async fn create(
        user: &entities::user::Model,
        db: &DatabaseConnection,
        request_data: RequestData,
    ) -> entities::push_subscription::Model {
        let push_subscription = entities::push_subscription::ActiveModel {
            endpoint: Set(request_data.endpoint),
            auth_key: Set(request_data.auth_key),
            p256dh_key: Set(request_data.p256dh_key),
            user_id: Set(user.id),
            ..Default::default()
        };

        PushSubscriptionsRepo::create_push_subscription(db, push_subscription).await
    }
}
