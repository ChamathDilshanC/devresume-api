use search::{
    build_fulltext_search_query, build_vector_cosine_query, compute_rrf_score, HybridSearchEngine,
    RankedItem,
};

#[test]
fn test_end_to_end_hybrid_search() {
    let engine = HybridSearchEngine::new(60);

    let keyword_results = vec![
        RankedItem {
            id: "doc-1".to_string(),
            entity_type: "project".to_string(),
            title: "DevResume API".to_string(),
            snippet: "Rust backend with Axum".to_string(),
            rank: 1,
            raw_score: 0.95,
        },
        RankedItem {
            id: "doc-2".to_string(),
            entity_type: "project".to_string(),
            title: "DevResume Web".to_string(),
            snippet: "Next.js frontend".to_string(),
            rank: 2,
            raw_score: 0.80,
        },
    ];

    let vector_results = vec![
        RankedItem {
            id: "doc-2".to_string(),
            entity_type: "project".to_string(),
            title: "DevResume Web".to_string(),
            snippet: "Next.js frontend".to_string(),
            rank: 1,
            raw_score: 0.98,
        },
        RankedItem {
            id: "doc-1".to_string(),
            entity_type: "project".to_string(),
            title: "DevResume API".to_string(),
            snippet: "Rust backend with Axum".to_string(),
            rank: 2,
            raw_score: 0.92,
        },
    ];

    let fused = engine.fuse_results(&keyword_results, &vector_results, None, 10);
    assert_eq!(fused.len(), 2);

    let doc1_score = compute_rrf_score(Some(1), Some(2), 60);
    let doc2_score = compute_rrf_score(Some(2), Some(1), 60);
    assert!((doc1_score - doc2_score).abs() < 1e-6);

    let fts_q = build_fulltext_search_query("projects", 5);
    let vec_q = build_vector_cosine_query("embeddings", 5);
    assert!(fts_q.contains("projects"));
    assert!(vec_q.contains("embeddings"));
}
