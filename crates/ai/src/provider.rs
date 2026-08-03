use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!("[OpenAI Response] Processed prompt: {}", prompt))
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

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!("[Gemini Response] Processed prompt: {}", prompt))
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

#[async_trait]
impl AIProvider for ClaudeProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!("[Claude Response] Processed prompt: {}", prompt))
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

#[async_trait]
impl AIProvider for OllamaProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        Ok(format!(
            "[Ollama Local Response] Processed prompt: {}",
            prompt
        ))
    }
    async fn embeddings(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.05; 1536])
    }
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {
        Ok(format!("[Ollama Chat] Handled {} messages", messages.len()))
    }
}
