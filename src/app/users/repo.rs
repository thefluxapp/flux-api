use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    Set,
};
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

use super::entities;

pub struct UsersRepo {}

impl UsersRepo {
    pub async fn create_user<T: ConnectionTrait>(
        db: &T,
        user_id: Uuid,
        email: String,
        first_name: String,
        last_name: String,
        passkey: Option<Passkey>,
    ) -> entities::user::Model {
        let user = entities::user::ActiveModel {
            id: Set(user_id),
            first_name: Set(Some(first_name)),
            last_name: Set(Some(last_name)),
            email: Set(email),
            passkeys: match passkey {
                Some(passkey) => Set(serde_json::to_value(vec![passkey]).unwrap()),
                None => NotSet,
            },
            created_at: NotSet,
            updated_at: NotSet,
        };

        user.insert(db).await.unwrap()
    }

    pub async fn find_user_by_id<T: ConnectionTrait>(
        db: &T,
        id: Uuid,
    ) -> Option<entities::user::Model> {
        entities::user::Entity::find_by_id(id)
            .one(db)
            .await
            .unwrap()
    }

    pub async fn find_user_by_email<T: ConnectionTrait>(
        db: &T,
        email: &String,
    ) -> Option<entities::user::Model> {
        entities::user::Entity::find()
            .filter(entities::user::Column::Email.eq(email))
            .one(db)
            .await
            .unwrap()
    }

    pub async fn update_user_passkeys<T: ConnectionTrait>(
        db: &T,
        mut user: entities::user::ActiveModel,
        passkeys: Vec<Passkey>,
    ) {
        user.passkeys = Set(serde_json::to_value(passkeys).unwrap());
        user.update(db).await.unwrap();
    }
}
