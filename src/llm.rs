use async_trait::async_trait;
use std::error::Error;

#[derive(Debug)]
pub struct Message<'a> {
    pub content: &'a str,
}

#[derive(Debug)]
pub struct Response {
    pub content: String,
}

#[async_trait]
pub trait LLM {
    async fn complete(&self, messages: &[Message]) -> Result<Response, Box<dyn Error>>;
}
