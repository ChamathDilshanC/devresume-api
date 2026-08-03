use crate::provider::{AIProvider, ChatMessage};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::warn;

pub struct FallbackAIProvider {
    pub providers: Vec<Arc<dyn AIProvider>>,
}

impl FallbackAIProvider {
    pub fn new(providers: Vec<Arc<dyn AIProvider>>) -> Self {
        Self { providers }
    }
}

#[async_trait]
impl AIProvider for FallbackAIProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        for (idx, provider) in self.providers.iter().enumerate() {
            match provider.generate(prompt).await {
                Ok(res) => return Ok(res),
                Err(err) => {
                    warn!(
                        "AI provider index {} failed with error: {}. Trying fallback provider...",
                        idx, err
                    );
                }
            }
        }

        Err(anyhow::anyhow!("All AI providers failed in fallback chain"))
    }

    async fn embeddings(&self, text: &str) -> Result<Vec<f32>> {
        for (idx, provider) in self.providers.iter().enumerate() {
            match provider.embeddings(text).await {
                Ok(res) => return Ok(res),
                Err(err) => {
                    warn!(
                        "AI embedding provider index {} failed with error: {}. Trying fallback...",
                        idx, err
                    );
                }
            }
        }

        Err(anyhow::anyhow!(
            "All embedding providers failed in fallback chain"
        ))
    }

    async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {
        for (idx, provider) in self.providers.iter().enumerate() {
            match provider.chat(messages).await {
                Ok(res) => return Ok(res),
                Err(err) => {
                    warn!(
                        "AI chat provider index {} failed with error: {}. Trying fallback...",
                        idx, err
                    );
                }
            }
        }

        Err(anyhow::anyhow!(
            "All chat providers failed in fallback chain"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::MockAIProvider;

    #[tokio::test]
    async fn test_fallback_provider_chain() {
        let primary = Arc::new(MockAIProvider);
        let fallback = FallbackAIProvider::new(vec![primary]);

        let res = fallback.generate("test prompt").await.unwrap();
        assert!(res.contains("Mock AI summary"));
    }
}
