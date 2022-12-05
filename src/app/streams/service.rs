use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use tokenizers::Tokenizer;
use tch::Tensor;

use super::entities;
use crate::app::User;

pub struct StreamsService {}

// TODO: ADD ON CONFLICT
impl StreamsService {
    pub async fn index(pool: &DatabaseConnection) -> Vec<entities::stream::Model> {
        entities::stream::Entity::find().all(pool).await.unwrap()
    }

    pub async fn show(
        stream_id: Uuid,
        pool: &DatabaseConnection,
        tokenizer: &Tokenizer,
    ) -> (
        entities::stream::Model,
        Vec<(entities::message::Model, Option<entities::stream::Model>)>,
    ) {
        let stream = entities::stream::Entity::find_by_id(stream_id)
            .one(pool)
            .await
            .unwrap()
            .unwrap();

        let sentence = "Google is an American multinational technology company focusing on search engine technology, online advertising, cloud computing, computer software, quantum computing, e-commerce, artificial intelligence, and consumer electronics. It has been referred to as \"the most powerful company in the world\" and one of the world's most valuable brands due to its market dominance, data collection, and technological advantages in the area of artificial intelligence. Its parent company Alphabet is considered one of the Big Five American information technology companies, alongside Amazon, Apple, Meta, and Microsoft.".to_string();
        let encoding = tokenizer
            .encode(sentence, false)
            .unwrap();


        let ids = encoding.get_ids();
        println!("{:?}", ids);

        let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
        let t = t * 2;
        t.print();

        // let messages = entities::message::Entity::find()
        //     .inner_join(entities::message_stream::Entity)
        //     .filter(entities::message_stream::Column::StreamId.eq(stream.id))
        //     .all(pool)
        //     .await
        //     .unwrap();

        let messages = StreamsService::find_messages_with_streams_by_stream(stream.id, pool).await;

        (stream, messages)
    }

    async fn find_messages_with_streams_by_stream(
        stream_id: Uuid,
        pool: &DatabaseConnection,
    ) -> Vec<(entities::message::Model, Option<entities::stream::Model>)> {
        entities::message::Entity::find()
            .inner_join(entities::message_stream::Entity)
            .find_also_related(entities::stream::Entity)
            .filter(entities::message_stream::Column::StreamId.eq(stream_id))
            .all(pool)
            .await
            .unwrap()
    }

    pub async fn find_or_create_by_user(
        user: &User,
        pool: &DatabaseConnection,
    ) -> entities::stream::Model {
        match entities::stream::Entity::find()
            .filter(entities::stream::Column::UserId.eq(user.id))
            .one(pool)
            .await
            .unwrap()
        {
            Some(stream) => stream,
            None => {
                let stream = entities::stream::ActiveModel {
                    user_id: Set(Some(user.id)),
                    ..Default::default()
                };

                stream.insert(pool).await.unwrap()
            }
        }
    }

    pub async fn find_or_create_by_message(
        message: entities::message::Model,
        pool: &DatabaseConnection,
    ) -> entities::stream::Model {
        match entities::stream::Entity::find()
            .filter(entities::stream::Column::MessageId.eq(message.id))
            .one(pool)
            .await
            .unwrap()
        {
            Some(stream) => stream,
            None => {
                let stream = entities::stream::ActiveModel {
                    message_id: Set(Some(message.id)),
                    ..Default::default()
                };

                stream.insert(pool).await.unwrap()
            }
        }
    }
}
