use crate::providers::traits::{LLMProvider, LLMRequest, LLMResponse};
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use reqwest::Client;

pub struct LlamaCppProvider {
    client: Client,
    endpoint: String,
}

impl LlamaCppProvider {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
        }
    }
}

#[async_trait]
impl LLMProvider for LlamaCppProvider {
    fn name(&self) -> &str {
        "LlamaCpp"
    }

    async fn generate(&self, request: LLMRequest) -> Result<LLMResponse> {
        let body = serde_json::json!({
            "prompt": format!("{} \n\n {}", request.system_prompt, request.prompt),
            "n_predict": request.max_tokens,
            "temperature": request.temperature,
        });

        let response = self.client.post(&format!("{}/completion", self.endpoint))
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let content = response["content"].as_str()
            .ok_or_else(|| anyhow!("Invalid response from llama.cpp"))?;

        Ok(LLMResponse { content: content.to_string() })
    }

    async fn stream_generate(&self, _request: LLMRequest) -> Result<tokio::io::AsyncRead> {
        todo!("Streaming implementation for llama.cpp")
    }
}
