use jobs::{
    AIJobPayload, AIWorker, CleanupWorker, EmbeddingJobPayload, EmbeddingWorker, Job,
    JobQueueEngine, JobType, SyncJobPayload, SyncWorker,
};

#[tokio::test]
async fn test_sync_worker_pipeline() {
    let payload = SyncJobPayload {
        user_id: "user-1".to_string(),
        github_token: "gho_test_token".to_string(),
        full_sync: true,
    };

    let result = SyncWorker::process_sync_job(&payload)
        .await
        .expect("Sync failed");
    assert_eq!(result.status, "completed");
}

#[tokio::test]
async fn test_embedding_worker_pipeline() {
    let payload = EmbeddingJobPayload {
        entity_id: "proj-1".to_string(),
        entity_type: "project".to_string(),
        content: "Rust backend engine".to_string(),
        model_name: "text-embedding-3-small".to_string(),
    };

    let result = EmbeddingWorker::process_embedding_job(&payload)
        .await
        .expect("Embedding failed");
    assert_eq!(result.chunks_embedded, 1);
}

#[tokio::test]
async fn test_ai_worker_pipeline() {
    let payload = AIJobPayload {
        task_type: "ats_analysis".to_string(),
        prompt: "Analyze ATS score".to_string(),
        provider_name: "OpenAI".to_string(),
    };

    let result = AIWorker::process_ai_job(&payload)
        .await
        .expect("AI job failed");
    assert_eq!(result.status, "completed");
}

#[tokio::test]
async fn test_queue_and_cleanup_workers() {
    let cleaned = CleanupWorker::run_cleanup_job()
        .await
        .expect("Cleanup failed");
    assert!(cleaned > 0);

    let mut queue = JobQueueEngine::new();
    let job = Job {
        id: "job-failed-1".to_string(),
        job_type: JobType::AI,
        payload_json: "{}".to_string(),
        retry_count: 2,
        max_retries: 3,
        status: "processing".to_string(),
        error_message: None,
    };

    queue.handle_job_failure(job, "Connection timeout");
    assert_eq!(queue.dead_letter_queue.len(), 1);
}
