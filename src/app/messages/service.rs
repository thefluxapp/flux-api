use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use super::data::{CreateData, IndexData};
use crate::app::{messages::entities::message, User};

pub struct MessagesService {}

impl MessagesService {
    pub fn index() -> IndexData {
        IndexData {
            messages: String::from("MESSAGES"),
        }
    }

    pub async fn create(user: User, pool: &DatabaseConnection) -> CreateData {
        let message = message::ActiveModel {
            text: Set(String::from("MESSAGE")),
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
