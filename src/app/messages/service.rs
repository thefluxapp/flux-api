use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use validator::Validate;

use super::{
    data::{CreateData, IndexData},
    payload::CreateMessagePayload,
};
use crate::app::{messages::entities::message, User};

pub struct MessagesService {}

impl MessagesService {
    pub fn index() -> IndexData {
        IndexData {
            messages: String::from("MESSAGES"),
        }
    }

    pub async fn create(
        user: User,
        pool: &DatabaseConnection,
        payload: CreateMessagePayload,
    ) -> CreateData {
        payload.validate().unwrap();

        let message = message::ActiveModel {
            text: Set(payload.text),
            user_id: Set(user.id),
            ..Default::default()
        };

        let message: message::Model = message.insert(pool).await.unwrap();

        CreateData {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {}
