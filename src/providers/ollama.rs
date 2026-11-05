use crate::providers::traits::{LLMProvider, LLMRequest, LLMResponse};
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use reqwest::Client;

pub struct OllamaProvider {
    client: Client,
    endpoint: String,
}

impl OllamaProvider {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }

    async fn generate(&self, request: LLMRequest) -> Result<LLMResponse> {
        let body = serde_json::json!({
            "model": "deepseek-coder", // Default or config
            "prompt": request.prompt,
            "system": request.system_prompt,
            "stream": false,
            "options": {
                "temperature": request.temperature,
                "num_predict": request.max_tokens,
            }
        });

        let response = self.client.post(&format!("{}/api/generate", self.endpoint))
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let content = response["response"].as_str()
            .ok_or_else(|| anyhow!("Invalid response from Ollama"))?;

        Ok(LLMResponse { content: content.to_string() })
    }

    async fn stream_generate(&self, _request: LLMRequest) -> Result<tokio::io::AsyncRead> {
        // Implementation for streaming...
        todo!("Streaming implementation for Ollama")
    }
}
