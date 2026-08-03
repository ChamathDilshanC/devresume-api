use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
    async fn embeddings(&self, text: &str) -> Result<Vec<f32>>;
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String>;
}

// 1. OpenAI Provider
pub struct OpenAIProvider {
    pub api_key: String,
    pub model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gpt-4o".to_string(),
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "[OpenAI Response - {}] Processed: {}",
            self.model, prompt
        ))
    }
    async fn embeddings(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.1; 1536])
    }
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {
        Ok(format!("[OpenAI Chat] Handled {} messages", messages.len()))
    }
}

// 2. Gemini Provider
pub struct GeminiProvider {
    pub api_key: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!("[Gemini Response] Processed: {}", prompt))
    }
    async fn embeddings(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.2; 1536])
    }
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {
        Ok(format!("[Gemini Chat] Handled {} messages", messages.len()))
    }
}

// 3. Claude Provider
pub struct ClaudeProvider {
    pub api_key: String,
}

impl ClaudeProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl AIProvider for ClaudeProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!("[Claude Response] Processed: {}", prompt))
    }
    async fn embeddings(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.3; 1536])
    }
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {
        Ok(format!("[Claude Chat] Handled {} messages", messages.len()))
    }
}

// 4. Ollama Provider (Local LLM)
pub struct OllamaProvider {
    pub base_url: String,
}

impl OllamaProvider {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait]
impl AIProvider for OllamaProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!("[Ollama Local] Processed: {}", prompt))
    }
    async fn embeddings(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.05; 1536])
    }
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {
        Ok(format!("[Ollama Chat] Handled {} messages", messages.len()))
    }
}

// 5. Mock Provider for Offline Tests
pub struct MockAIProvider;

#[async_trait]
impl AIProvider for MockAIProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "{{\"summary\":\"Mock AI summary for {}\",\"maturity\":\"stable\"}}",
            prompt
        ))
    }
    async fn embeddings(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.0; 1536])
    }
    async fn chat(&self, _messages: &[ChatMessage]) -> Result<String> {
        Ok("Mock Chat Response".to_string())
    }
}

// Factory Function
pub fn create_ai_provider(provider_type: &str, secret_or_url: &str) -> Arc<dyn AIProvider> {
    match provider_type.to_lowercase().as_str() {
        "openai" => Arc::new(OpenAIProvider::new(secret_or_url.to_string())),
        "gemini" => Arc::new(GeminiProvider::new(secret_or_url.to_string())),
        "claude" => Arc::new(ClaudeProvider::new(secret_or_url.to_string())),
        "ollama" => Arc::new(OllamaProvider::new(secret_or_url.to_string())),
        _ => Arc::new(MockAIProvider),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_factory_creates_providers() {
        let openai = create_ai_provider("openai", "key");
        let res = openai.generate("test").await.unwrap();
        assert!(res.contains("OpenAI"));

        let mock = create_ai_provider("mock", "");
        let emb = mock.embeddings("hello").await.unwrap();
        assert_eq!(emb.len(), 1536);
    }
}
