use std::{env, time::Duration};

use axum::BoxError;
use reqwest::{header::AUTHORIZATION, Client};
use serde::Deserialize;

mod ya_gpt;

pub struct Summarizer {
    client: Client,
    ya_gpt: YaGPT,
}

impl Summarizer {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
            ya_gpt: YaGPT::new(),
        }
    }

    pub async fn ya_gpt_completion(&self, text: String) -> Result<String, BoxError> {
        let request = ya_gpt::Request::new(
            self.ya_gpt.model_url.clone(),
            self.ya_gpt.temperature,
            self.ya_gpt.max_tokens,
            self.ya_gpt.instruction.clone(),
            text,
        );

        let authorization = ["Authorization: Api-Key", self.ya_gpt.api_key.as_str()].join(" ");

        dbg!(&request);

        let result = self
            .client
            .post(&self.ya_gpt.url)
            .header(AUTHORIZATION, authorization)
            .json(&request)
            .send()
            .await?;

        let response = match result.error_for_status() {
            Ok(result) => match result.json::<ya_gpt::Response>().await {
                Ok(result) => result.text(),
                Err(err) => return Err(Box::new(err)),
            },
            Err(err) => return Err(Box::new(err)),
        };

        Ok(response)
    }

    // pub async fn call(&self, text: String) -> Result<MyData, BoxError> {
    //     let mut map = HashMap::new();
    //     map.insert("text", &text);

    //     let res = self
    //         .client
    //         .post(env::var("FLUX_AI_URL").unwrap())
    //         .json(&map)
    //         .send()
    //         .await?
    //         .json::<MyData>()
    //         .await
    //         .unwrap();

    //     return Ok(res);
    // }
}

struct YaGPT {
    url: String,
    api_key: String,
    model_url: String,
    temperature: f32,
    instruction: String,
    max_tokens: i32,
}

impl YaGPT {
    pub fn new() -> Self {
        Self {
            url: env::var("YA_GPT_URL").unwrap(),
            model_url: env::var("YA_GPT_MODEL_URI").unwrap(),
            api_key: env::var("YA_GPT_API_KEY").unwrap(),
            temperature: env::var("YA_GPT_TEMPERATURE")
                .unwrap_or("0.6".to_string())
                .parse()
                .unwrap(),
            instruction: env::var("YA_GPT_INSTRUCTION").unwrap(),
            max_tokens: env::var("YA_GPT_MAX_TOKENS")
                .unwrap_or("8000".to_string())
                .parse()
                .unwrap(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MyData {
    pub text: String,
}
