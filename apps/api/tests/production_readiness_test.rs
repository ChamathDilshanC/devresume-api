use ai::{create_ai_provider, AIProvider, FallbackAIProvider, MockAIProvider};
use std::sync::Arc;

#[tokio::test]
async fn test_production_ai_fallback_chain() {
    let mock_provider = Arc::new(MockAIProvider);
    let fallback = FallbackAIProvider::new(vec![mock_provider]);

    let res = fallback
        .generate("Analyze repo")
        .await
        .expect("Fallback generation failed");
    assert!(res.contains("Mock AI summary"));
}

#[tokio::test]
async fn test_production_multi_provider_factory() {
    let openai = create_ai_provider("openai", "mock-key");
    let gemini = create_ai_provider("gemini", "mock-key");
    let claude = create_ai_provider("claude", "mock-key");

    let chain = FallbackAIProvider::new(vec![openai, gemini, claude]);
    let result = chain
        .generate("Test prompt")
        .await
        .expect("Chain execution failed");

    assert!(!result.is_empty());
}
