use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommitSyncError {
    #[error("Client build error: {0}")]
    ClientBuild(reqwest::Error),
    #[error("HTTP request error: {0}")]
    RequestFailed(reqwest::Error),
    #[error("API error {0}: {1}")]
    ApiError(u16, String),
    #[error("JSON decode error: {0}")]
    JsonDecode(reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitStats {
    pub hash: String,
    pub author_name: String,
    pub message: String,
    pub additions: i32,
    pub deletions: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubCommitDetail {
    pub author: Option<GitHubCommitAuthor>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubCommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubCommit {
    pub sha: String,
    pub commit: GitHubCommitDetail,
    pub html_url: String,
}

pub fn calculate_commit_impact(additions: i32, deletions: i32) -> f64 {
    let total = (additions + deletions) as f64;
    (total * 0.1).min(100.0)
}

pub struct GithubCommitClient {
    client: Client,
    base_url: String,
}

impl GithubCommitClient {
    pub fn new() -> Result<Self, CommitSyncError> {
        Self::with_base_url("https://api.github.com".to_string())
    }

    pub fn with_base_url(base_url: String) -> Result<Self, CommitSyncError> {
        let client = Client::builder()
            .user_agent("DevResume-AI")
            .build()
            .map_err(CommitSyncError::ClientBuild)?;

        Ok(Self { client, base_url })
    }

    pub async fn fetch_commits(
        &self,
        token: &str,
        owner: &str,
        repo: &str,
        since: Option<&str>,
    ) -> Result<Vec<GitHubCommit>, CommitSyncError> {
        let mut url = format!(
            "{}/repos/{}/{}/commits?per_page=100",
            self.base_url, owner, repo
        );
        if let Some(since_ts) = since {
            url.push_str(&format!("&since={}", urlencoding::encode(since_ts)));
        }

        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(CommitSyncError::RequestFailed)?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(CommitSyncError::ApiError(status, text));
        }

        response
            .json::<Vec<GitHubCommit>>()
            .await
            .map_err(CommitSyncError::JsonDecode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_commit_impact() {
        let impact = calculate_commit_impact(50, 10);
        assert_eq!(impact, 6.0);

        let capped_impact = calculate_commit_impact(2000, 1000);
        assert_eq!(capped_impact, 100.0);
    }
}
