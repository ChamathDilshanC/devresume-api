use ai::{
    build_project_summary_prompt, create_ai_provider, generate_chunk_embedding_vector,
    ChunkingEngine, EmbeddingModel, ProjectContext,
};
use auth::{create_jwt, verify_jwt};
use jobs::{
    AIJobPayload, AIWorker, EmbeddingJobPayload, EmbeddingWorker, SyncJobPayload, SyncWorker,
};
use parser::{
    detect_architecture_pattern, detect_technologies_from_files, parse_cargo_toml, parse_readme,
    ArchitecturePattern,
};
use search::{HybridSearchEngine, RankedItem};
use uuid::Uuid;

#[tokio::test]
async fn test_complete_end_to_end_ai_pipeline_journey() {
    // 1. User Login & JWT Token Generation
    let user_id = Uuid::new_v4();
    let email = "developer@devresume.ai";
    let secret = "supersecretkey123";

    let token = create_jwt(user_id, email, secret).expect("JWT creation failed");
    let verified_claims = verify_jwt(&token, secret).expect("JWT verification failed");

    assert_eq!(verified_claims.sub, user_id.to_string());
    assert_eq!(verified_claims.email, email);

    // 2. Connect GitHub & Repository Sync
    let sync_payload = SyncJobPayload {
        user_id: user_id.to_string(),
        github_token: "gho_mock_e2e_token".to_string(),
        full_sync: true,
    };
    let sync_result = SyncWorker::process_sync_job(&sync_payload)
        .await
        .expect("Sync failed");
    assert_eq!(sync_result.status, "completed");

    // 3. Webhook Trigger & Repository Parsing
    let repo_files = vec![
        (
            "Cargo.toml",
            "[package]\nname=\"devresume-api\"\n[dependencies]\naxum=\"0.7\"\nsqlx=\"0.7\"\ntokio=\"1.0\"\n",
        ),
        (
            "package.json",
            "{\"name\":\"web\",\"dependencies\":{\"next\":\"14.0.0\"},\"devDependencies\":{\"typescript\":\"5.0.0\"}}",
        ),
        (
            "Dockerfile",
            "FROM rust:1.78 as builder\nEXPOSE 8080\nFROM debian:bookworm-slim\n",
        ),
        (
            "README.md",
            "# DevResume AI\nAI-powered developer resume & portfolio platform built with Rust.\n",
        ),
    ];

    let cargo = parse_cargo_toml(repo_files[0].1);
    assert_eq!(cargo.package_name, Some("devresume-api".to_string()));

    let readme = parse_readme(repo_files[3].1);
    assert_eq!(readme.title, Some("DevResume AI".to_string()));

    // 4. Technology & Architecture Detection
    let tech_profile = detect_technologies_from_files(&repo_files);
    assert!(tech_profile.languages.contains(&"Rust".to_string()));
    assert!(tech_profile.languages.contains(&"TypeScript".to_string()));
    assert!(tech_profile.frameworks.contains(&"Axum/Actix".to_string()));
    assert!(tech_profile.frameworks.contains(&"Next.js".to_string()));

    let filenames: Vec<&str> = repo_files.iter().map(|(name, _)| *name).collect();
    let arch_pattern = detect_architecture_pattern(&filenames);
    assert_eq!(arch_pattern, ArchitecturePattern::Monolith);

    // 5. Chunking & Embedding Generation
    let engine = ChunkingEngine::new(20, 5);
    let chunks = engine.chunk_text(repo_files[3].1);
    assert!(!chunks.is_empty());

    let embedding_payload = EmbeddingJobPayload {
        entity_id: "repo-devresume".to_string(),
        entity_type: "project".to_string(),
        content: chunks[0].content.clone(),
        model_name: "text-embedding-3-small".to_string(),
    };
    let emb_result = EmbeddingWorker::process_embedding_job(&embedding_payload)
        .await
        .expect("Embedding failed");
    assert_eq!(emb_result.status, "completed");

    let vector = generate_chunk_embedding_vector(
        &chunks[0].content,
        EmbeddingModel::OpenAiTextEmbedding3Small,
    );
    assert_eq!(vector.len(), 1536);

    // 6. Hybrid RRF Search
    let search_engine = HybridSearchEngine::new(60);
    let kw_results = vec![RankedItem {
        id: "repo-devresume".to_string(),
        entity_type: "project".to_string(),
        title: "DevResume AI".to_string(),
        snippet: chunks[0].content.clone(),
        rank: 1,
        raw_score: 0.95,
    }];
    let vec_results = vec![RankedItem {
        id: "repo-devresume".to_string(),
        entity_type: "project".to_string(),
        title: "DevResume AI".to_string(),
        snippet: chunks[0].content.clone(),
        rank: 1,
        raw_score: 0.98,
    }];
    let fused = search_engine.fuse_results(&kw_results, &vec_results, None, 5);
    assert_eq!(fused.len(), 1);
    assert_eq!(fused[0].id, "repo-devresume");

    // 7. AI Summary Generation
    let project_ctx = ProjectContext {
        name: "DevResume AI".to_string(),
        description: Some("AI platform".to_string()),
        primary_language: Some("Rust".to_string()),
        tech_stack: tech_profile.languages.clone(),
        readme_snippet: Some(readme.summary.clone()),
    };
    let prompt = build_project_summary_prompt(&project_ctx);
    let ai_provider = create_ai_provider("mock", "");
    let summary_response = ai_provider
        .generate(&prompt)
        .await
        .expect("AI summary generation failed");

    assert!(summary_response.contains("Mock AI summary"));

    // 8. Background AI Job Dispatch Validation
    let ai_job_payload = AIJobPayload {
        task_type: "summarize".to_string(),
        prompt: prompt.clone(),
        provider_name: "MockAI".to_string(),
    };
    let ai_job_result = AIWorker::process_ai_job(&ai_job_payload)
        .await
        .expect("AI worker job failed");
    assert_eq!(ai_job_result.status, "completed");
}
