use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use tokenizers::Tokenizer;
use validator::Validate;

use super::{
    data::{CreateData, IndexData},
    entities,
    payload::CreateMessagePayload,
};
use crate::app::{streams::service::StreamsService, User};

pub struct MessagesService {}

impl MessagesService {
    pub fn index() -> IndexData {
        IndexData {
            messages: String::from("MESSAGES"),
        }
    }

    pub async fn create(
        user: &User,
        pool: &DatabaseConnection,
        payload: CreateMessagePayload,
    ) -> CreateData {
        payload.validate().unwrap();

        let stream = if let Some(message_id) = payload.message_id {
            let message = entities::message::Entity::find_by_id(message_id)
                .one(pool)
                .await
                .unwrap()
                .unwrap();

            StreamsService::find_or_create_by_message(message, pool).await
        } else if let Some(stream_id) = payload.stream_id {
            entities::stream::Entity::find_by_id(stream_id)
                .one(pool)
                .await
                .unwrap()
                .unwrap()
        } else {
            StreamsService::find_or_create_by_user(user, pool).await
        };

        let message = MessagesService::create_with_stream(user, pool, payload, &stream).await;

        CreateData {
            message: message.into(),
        }
    }

    async fn create_with_stream(
        user: &User,
        pool: &DatabaseConnection,
        payload: CreateMessagePayload,
        stream: &entities::stream::Model,
    ) -> entities::message::Model {
        let txn = pool.begin().await.unwrap();

        let message = entities::message::ActiveModel {
            text: Set(payload.text),
            user_id: Set(user.id),
            ..Default::default()
        };

        let message: entities::message::Model = message.insert(&txn).await.unwrap();

        entities::message_stream::ActiveModel {
            message_id: Set(message.id),
            stream_id: Set(stream.id),
            ..Default::default()
        }
        .save(&txn)
        .await
        .unwrap();

        txn.commit().await.unwrap();

        message
    }

    pub async fn summarize(messages: Vec<entities::message::Model>) {
        // let raw_text: Vec<String> = messages.into_iter().map(|message| message.text).collect();

        // use rust_bert::pipelines::summarization::SummarizationModel;

        let sentence = "Google is an American multinational technology company focusing on search engine technology, online advertising, cloud computing, computer software, quantum computing, e-commerce, artificial intelligence, and consumer electronics. It has been referred to as \"the most powerful company in the world\" and one of the world's most valuable brands due to its market dominance, data collection, and technological advantages in the area of artificial intelligence. Its parent company Alphabet is considered one of the Big Five American information technology companies, alongside Amazon, Apple, Meta, and Microsoft.".to_string();
        let tokenizer = Tokenizer::from_pretrained("JulesBelveze/t5-small-headline-generator", None).unwrap();
        let encoding = tokenizer
            .encode(sentence, false)
            .unwrap();

        // let ids = encoding.get_ids();

        // println!("{:?}", ids);

        // let mut model = SummarizationModel::new(Default::default()).unwrap();


        // let output = model.summarize(&input);

        // dbg!(output);
        // let encoding = tokenizer.encode(raw_text, false).unwrap();
        // println!("{:?}", encoding.get_tokens());

        // let mut summariser =
        //     Summariser::from_raw_text(raw_text.clone(), ".", 5, 1500, false, false, 6);

        // dbg!(qq.join(". "));
    }
}

#[cfg(test)]
mod tests {
    use tokenizers::Tokenizer;

    #[test]
    fn test_summarize() {
        let sentence = "Google is an American multinational technology company focusing on search engine technology, online advertising, cloud computing, computer software, quantum computing, e-commerce, artificial intelligence, and consumer electronics. It has been referred to as \"the most powerful company in the world\" and one of the world's most valuable brands due to its market dominance, data collection, and technological advantages in the area of artificial intelligence. Its parent company Alphabet is considered one of the Big Five American information technology companies, alongside Amazon, Apple, Meta, and Microsoft.".to_string();
        let mut tokenizer = Tokenizer::from_pretrained("JulesBelveze/t5-small-headline-generator", None).unwrap();
        let encoding = tokenizer
            .encode(sentence, false)
            .unwrap();

        let ids = encoding.get_ids();

        println!("{:?}", ids);
    }
}
