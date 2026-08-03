use ai::{
    build_project_summary_prompt, cosine_similarity, create_ai_provider,
    generate_chunk_embedding_vector, ChunkingEngine, EmbeddingModel, ProjectContext,
};

#[tokio::test]
async fn test_ai_provider_factory_and_generation() {
    let mock = create_ai_provider("mock", "");
    let prompt = "Generate technical summary";
    let response = mock.generate(prompt).await.expect("Generation failed");

    assert!(response.contains("Mock AI summary"));
}

#[tokio::test]
async fn test_dynamic_embedding_dimensions() {
    let small_vec =
        generate_chunk_embedding_vector("Rust web app", EmbeddingModel::OpenAiTextEmbedding3Small);
    let large_vec =
        generate_chunk_embedding_vector("Rust web app", EmbeddingModel::OpenAiTextEmbedding3Large);
    let nomic_vec =
        generate_chunk_embedding_vector("Rust web app", EmbeddingModel::OllamaNomicEmbedText);

    assert_eq!(small_vec.len(), 1536);
    assert_eq!(large_vec.len(), 3072);
    assert_eq!(nomic_vec.len(), 768);
}

#[tokio::test]
async fn test_chunking_and_similarity() {
    let text =
        "DevResume AI is a production platform built with Rust, Axum, PostgreSQL, and Next.js.";
    let engine = ChunkingEngine::new(5, 2);
    let chunks = engine.chunk_text(text);

    assert!(!chunks.is_empty());

    let v1 = generate_chunk_embedding_vector(&chunks[0].content, EmbeddingModel::MockModel);
    let v2 = generate_chunk_embedding_vector(&chunks[0].content, EmbeddingModel::MockModel);

    let sim = cosine_similarity(&v1, &v2);
    assert!((sim - 1.0).abs() < 1e-5);
}

#[tokio::test]
async fn test_project_context_prompt_building() {
    let ctx = ProjectContext {
        name: "DevResume".to_string(),
        description: Some("AI engine".to_string()),
        primary_language: Some("Rust".to_string()),
        tech_stack: vec!["Axum".to_string()],
        readme_snippet: Some("README".to_string()),
    };

    let prompt = build_project_summary_prompt(&ctx);
    assert!(prompt.contains("DevResume"));
}
