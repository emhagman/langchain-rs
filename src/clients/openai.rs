use std::error::Error;

use crate::llm::{Message as LLMMessage, Response, LLM};
use async_trait::async_trait;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

const OPENAI_BASE_URL: &str = "https://api.openai.com/v1/";
const OPENAI_MODEL: &str = "gpt-3.5-turbo";

pub struct OpenAILLM {
    client: Client,
}

#[async_trait]
impl LLM for OpenAILLM {
    async fn complete(&self, messages: &[LLMMessage]) -> Result<Response, Box<dyn Error>> {
        let response = self.completion(messages).await?;
        Ok(Response { content: response })
    }
}

#[derive(Serialize)]
struct CompletionRequestMessage<'a> {
    role: String,
    content: &'a str,
}

#[derive(Serialize)]
struct CompletionRequest<'a> {
    model: String,
    temperature: f64,
    messages: Vec<CompletionRequestMessage<'a>>,
}

impl OpenAILLM {
    pub fn new(api_key: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.append(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", &api_key)).unwrap(),
        );
        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        OpenAILLM { client }
    }

    fn convert_messages<'a>(&self, messages: &[LLMMessage<'a>]) -> Vec<CompletionRequestMessage<'a>> {
        messages
            .iter()
            .map(|f| CompletionRequestMessage {
                role: "user".to_string(),
                content: f.content,
            })
            .collect()
    }

    async fn completion(&self, messages: &[LLMMessage<'_>]) -> Result<String, Box<dyn Error>> {
        let messages = self.convert_messages(messages);
        let request = CompletionRequest {
            model: OPENAI_MODEL.to_string(),
            temperature: 0.1,
            messages,
        };
        let completion_url = format!("{}/chat/completions", OPENAI_BASE_URL);
        let body = self
            .client
            .post(&completion_url)
            .json(&request)
            .send()
            .await?
            .json::<OpenAIResponse>()
            .await?;

        let message = &body.choices[0].message.content;
        Ok(message.to_string())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
