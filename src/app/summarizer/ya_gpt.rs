use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    model_uri: String,
    completion_options: CompletionOptions,
    messages: Vec<Message>,
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
    User,
    Assistant,
}

impl Request {
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
#[serde(rename_all = "snake_case")]
pub struct Response {
    result: Result,
}

impl Response {
    pub fn text(&self) -> String {
        self.result
            .alternatives
            .first()
            .unwrap()
            .message
            .text
            .clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Result {
    alternatives: Vec<Alternative>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Alternative {
    message: Message,
}
