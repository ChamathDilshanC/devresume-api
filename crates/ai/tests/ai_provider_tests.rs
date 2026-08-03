use ai::{
    build_project_summary_prompt, cosine_similarity, create_ai_provider,
    generate_chunk_embedding_vector, ProjectContext,
};

#[tokio::test]
async fn test_ai_provider_factory_and_generation() {
    let mock = create_ai_provider("mock", "");
    let prompt = "Generate technical summary";
    let response = mock.generate(prompt).await.expect("Generation failed");

    assert!(response.contains("Mock AI summary"));
}

#[tokio::test]
async fn test_embedding_generation_and_similarity() {
    let text1 = "Rust web backend developer using Axum and PostgreSQL";
    let text2 = "Rust web backend developer using Axum and PostgreSQL";

    let v1 = generate_chunk_embedding_vector(text1);
    let v2 = generate_chunk_embedding_vector(text2);

    assert_eq!(v1.len(), 1536);
    let sim = cosine_similarity(&v1, &v2);
    assert!((sim - 1.0).abs() < 1e-5);
}

#[tokio::test]
async fn test_prompt_building_and_provider_flow() {
    let ctx = ProjectContext {
        name: "devresume-api".to_string(),
        description: Some("Production Rust API".to_string()),
        primary_language: Some("Rust".to_string()),
        tech_stack: vec![
            "Axum".to_string(),
            "SQLx".to_string(),
            "PostgreSQL".to_string(),
        ],
        readme_snippet: Some("Modular monolith architecture".to_string()),
    };

    let prompt = build_project_summary_prompt(&ctx);
    let provider = create_ai_provider("mock", "");
    let response = provider.generate(&prompt).await.expect("Generation failed");

    assert!(!response.is_empty());
}
