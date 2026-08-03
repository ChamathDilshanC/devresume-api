use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingJobPayload {
    pub entity_id: String,
    pub entity_type: String, // "project", "code", "document"
    pub content: String,
    pub model_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingJobResult {
    pub entity_id: String,
    pub chunks_embedded: usize,
    pub vector_dimensions: usize,
    pub status: String,
}

pub struct EmbeddingWorker;

impl EmbeddingWorker {
    pub async fn process_embedding_job(
        payload: &EmbeddingJobPayload,
    ) -> Result<EmbeddingJobResult, String> {
        if payload.content.is_empty() {
            return Err("Empty content provided for embedding".to_string());
        }

        let words: Vec<&str> = payload.content.split_whitespace().collect();
        let chunk_count = (words.len() / 100).max(1);

        Ok(EmbeddingJobResult {
            entity_id: payload.entity_id.clone(),
            chunks_embedded: chunk_count,
            vector_dimensions: 1536,
            status: "completed".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedding_worker_success() {
        let payload = EmbeddingJobPayload {
            entity_id: "proj-99".to_string(),
            entity_type: "project".to_string(),
            content: "Rust Axum PostgreSQL backend architecture".to_string(),
            model_name: "text-embedding-3-small".to_string(),
        };

        let result = EmbeddingWorker::process_embedding_job(&payload)
            .await
            .unwrap();
        assert_eq!(result.status, "completed");
        assert_eq!(result.vector_dimensions, 1536);
    }
}
