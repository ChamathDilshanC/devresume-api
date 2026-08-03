use crate::commits::GithubCommitClient;
use crate::repositories::{GithubRepo, GithubRepoClient, SyncOptions};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Repository fetch failed: {0}")]
    RepoError(#[from] crate::repositories::GitHubRepoError),
    #[error("Commit fetch failed: {0}")]
    CommitError(#[from] crate::commits::CommitSyncError),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SyncType {
    Full,
    Incremental,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositorySyncReport {
    pub total_repositories_synced: usize,
    pub total_commits_synced: usize,
    pub sync_type: SyncType,
    pub duration_seconds: f64,
}

pub struct RepositorySyncEngine {
    repo_client: GithubRepoClient,
    commit_client: GithubCommitClient,
}

impl RepositorySyncEngine {
    pub fn new() -> Result<Self, SyncError> {
        Ok(Self {
            repo_client: GithubRepoClient::new()?,
            commit_client: GithubCommitClient::new()?,
        })
    }

    pub fn with_clients(repo_client: GithubRepoClient, commit_client: GithubCommitClient) -> Self {
        Self {
            repo_client,
            commit_client,
        }
    }

    pub async fn sync_user_repositories(
        &self,
        token: &str,
        sync_type: SyncType,
    ) -> Result<RepositorySyncReport, SyncError> {
        let start = std::time::Instant::now();
        let opts = SyncOptions::default();

        let repos: Vec<GithubRepo> = self.repo_client.fetch_repositories(token, &opts).await?;
        let repo_count = repos.len();
        let mut total_commits = 0;

        for repo in &repos {
            let owner_and_name: Vec<&str> = repo.full_name.split('/').collect();
            if owner_and_name.len() == 2 {
                let owner = owner_and_name[0];
                let repo_name = owner_and_name[1];

                let since = if sync_type == SyncType::Incremental {
                    repo.updated_at.as_deref()
                } else {
                    None
                };

                if let Ok(commits) = self
                    .commit_client
                    .fetch_commits(token, owner, repo_name, since)
                    .await
                {
                    total_commits += commits.len();
                }
            }
        }

        let duration = start.elapsed().as_secs_f64();

        Ok(RepositorySyncReport {
            total_repositories_synced: repo_count,
            total_commits_synced: total_commits,
            sync_type,
            duration_seconds: duration,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_engine_instantiation() {
        let engine = RepositorySyncEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_sync_report_format() {
        let report = RepositorySyncReport {
            total_repositories_synced: 5,
            total_commits_synced: 120,
            sync_type: SyncType::Full,
            duration_seconds: 1.5,
        };

        assert_eq!(report.total_repositories_synced, 5);
        assert_eq!(report.sync_type, SyncType::Full);
    }
}
