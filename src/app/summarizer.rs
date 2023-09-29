use std::{collections::HashMap, env};

use reqwest::{Client, Error};
use serde::Deserialize;

pub struct Summarizer {
    client: Client,
}

impl Summarizer {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn call(&self, text: String) -> Result<MyData, Error> {
        let mut map = HashMap::new();
        map.insert("text", &text);

        let res = self
            .client
            .post(env::var("FLUX_AI_URL").unwrap())
            .json(&map)
            .send()
            .await?
            .json::<MyData>()
            .await
            .unwrap();

        return Ok(res);
    }
}

#[derive(Deserialize, Debug)]
pub struct MyData {
    pub text: String,
}
