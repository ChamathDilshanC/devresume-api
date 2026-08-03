use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIJobPayload {
    pub task_type: String, // "summarize", "ats_analysis", "bullet_points"
    pub prompt: String,
    pub provider_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIJobResult {
    pub task_type: String,
    pub response: String,
    pub tokens_used: usize,
    pub status: String,
}

pub struct AIWorker;

impl AIWorker {
    pub async fn process_ai_job(payload: &AIJobPayload) -> Result<AIJobResult, String> {
        if payload.prompt.is_empty() {
            return Err("Empty AI prompt provided".to_string());
        }

        Ok(AIJobResult {
            task_type: payload.task_type.clone(),
            response: format!(
                "[Processed by {}] Output for {}",
                payload.provider_name, payload.task_type
            ),
            tokens_used: 240,
            status: "completed".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_worker_success() {
        let payload = AIJobPayload {
            task_type: "summarize".to_string(),
            prompt: "Summarize repository".to_string(),
            provider_name: "OpenAI".to_string(),
        };

        let result = AIWorker::process_ai_job(&payload).await.unwrap();
        assert_eq!(result.status, "completed");
        assert!(result.response.contains("OpenAI"));
    }
}
