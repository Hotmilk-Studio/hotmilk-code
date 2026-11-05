use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub system_prompt: String,
    pub temperature: f32,
    pub max_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
}

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn generate(&self, request: LLMRequest) -> Result<LLMResponse>;
    async fn stream_generate(&self, request: LLMRequest) -> Result<tokio::io::AsyncRead>;
    fn name(&self) -> &str;
}
