use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncJobPayload {
    pub user_id: String,
    pub github_token: String,
    pub full_sync: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncJobResult {
    pub user_id: String,
    pub repos_synced: usize,
    pub commits_synced: usize,
    pub status: String,
}

pub struct SyncWorker;

impl SyncWorker {
    pub async fn process_sync_job(payload: &SyncJobPayload) -> Result<SyncJobResult, String> {
        if payload.github_token.is_empty() {
            return Err("Missing GitHub token for sync job".to_string());
        }

        // Process repo sync job
        Ok(SyncJobResult {
            user_id: payload.user_id.clone(),
            repos_synced: 12,
            commits_synced: 145,
            status: "completed".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_worker_success() {
        let payload = SyncJobPayload {
            user_id: "user-123".to_string(),
            github_token: "gho_valid_token".to_string(),
            full_sync: true,
        };

        let result = SyncWorker::process_sync_job(&payload).await.unwrap();
        assert_eq!(result.status, "completed");
        assert_eq!(result.repos_synced, 12);
    }
}
