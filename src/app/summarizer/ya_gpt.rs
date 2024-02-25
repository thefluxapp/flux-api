use std::{env, time::Duration};

use axum::BoxError;
use reqwest::{header::AUTHORIZATION, Client};
use serde::{Deserialize, Serialize};

pub struct YaGPT {
    completion_url: String,
    operation_url: String,
    api_key: String,
    model_url: String,
    temperature: f32,
    instruction: String,
    max_tokens: i32,
    client: Client,
}

impl YaGPT {
    pub fn new() -> Self {
        Self {
            completion_url: env::var("YA_GPT_URL").unwrap(),
            operation_url: env::var("YA_GPT_OPERATION_URL").unwrap(),
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
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(20))
                .build()
                .unwrap(),
        }
    }

    pub async fn ya_gpt_completion(&self, text: String) -> Result<String, BoxError> {
        let request = CompletionRequest::new(
            self.model_url.clone(),
            self.temperature,
            self.max_tokens,
            self.instruction.clone(),
            text,
        );

        let response = self
            .client
            .post(&self.completion_url)
            .header(AUTHORIZATION, self.ya_gpt_authorization())
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<CompletionResponse>()
            .await?;

        dbg!(&response);

        Ok(response.id)
    }

    pub async fn ya_gpt_operation(&self, ya_gpt_id: String) -> Result<String, BoxError> {
        let url = [self.operation_url.clone().to_owned(), ya_gpt_id].join("/");

        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, self.ya_gpt_authorization())
            .send()
            .await?
            .error_for_status()?
            .json::<OperationResponse>()
            .await?;

        dbg!(&response);

        match response.response {
            Some(response) => Ok(response.text()),
            None => Err("YaGPT_ERROR".into()),
        }
    }

    fn ya_gpt_authorization(&self) -> String {
        ["Authorization: Api-Key", self.api_key.as_str()].join(" ")
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CompletionOptions {
    stream: bool,
    temperature: f32,
    max_tokens: i32,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    role: Role,
    text: String,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    System,
    Dummy,
    User,
    Assistant,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionRequest {
    model_uri: String,
    completion_options: CompletionOptions,
    messages: Vec<Message>,
}

impl CompletionRequest {
    pub fn new(
        model_uri: String,
        temperature: f32,
        max_tokens: i32,
        instruction: String,
        text: String,
    ) -> Self {
        Self {
            model_uri,
            completion_options: CompletionOptions {
                stream: false,
                temperature,
                max_tokens,
            },
            messages: vec![
                Message {
                    role: Role::System,
                    text: instruction,
                },
                Message {
                    role: Role::User,
                    text,
                },
            ],
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub done: bool,
    // pub response: Option<Result>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OperationResponse {
    response: Option<OperationResponseResponse>,
}

impl OperationResponseResponse {
    pub fn text(&self) -> String {
        self.alternatives.first().unwrap().message.text.clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OperationResponseResponse {
    alternatives: Vec<OperationResponseAlternative>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct OperationResponseAlternative {
    message: Message,
}
