use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubRepoError {
    #[error("Client creation failed: {0}")]
    ClientBuild(reqwest::Error),
    #[error("HTTP request failed: {0}")]
    RequestFailed(reqwest::Error),
    #[error("API returned error status {0}: {1}")]
    ApiError(u16, String),
    #[error("Deserialization failed: {0}")]
    JsonParse(reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct GithubRepo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub default_branch: String,
    pub private: bool,
    pub language: Option<String>,
    pub stargazers_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SyncOptions {
    pub page: u32,
    pub per_page: u32,
    pub visibility: String,
    pub sort: String,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 30,
            visibility: "all".to_string(),
            sort: "updated".to_string(),
        }
    }
}

pub struct GithubRepoClient {
    client: Client,
    base_url: String,
}

impl GithubRepoClient {
    pub fn new() -> Result<Self, GitHubRepoError> {
        Self::with_base_url("https://api.github.com".to_string())
    }

    pub fn with_base_url(base_url: String) -> Result<Self, GitHubRepoError> {
        let client = Client::builder()
            .user_agent("DevResume-AI")
            .build()
            .map_err(GitHubRepoError::ClientBuild)?;

        Ok(Self { client, base_url })
    }

    pub async fn fetch_repositories(
        &self,
        token: &str,
        opts: &SyncOptions,
    ) -> Result<Vec<GithubRepo>, GitHubRepoError> {
        let url = format!(
            "{}/user/repos?page={}&per_page={}&visibility={}&sort={}",
            self.base_url, opts.page, opts.per_page, opts.visibility, opts.sort
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(GitHubRepoError::RequestFailed)?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(GitHubRepoError::ApiError(status, text));
        }

        response
            .json::<Vec<GithubRepo>>()
            .await
            .map_err(GitHubRepoError::JsonParse)
    }

    pub async fn fetch_single_repository(
        &self,
        token: &str,
        owner: &str,
        repo: &str,
    ) -> Result<GithubRepo, GitHubRepoError> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);

        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(GitHubRepoError::RequestFailed)?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(GitHubRepoError::ApiError(status, text));
        }

        response
            .json::<GithubRepo>()
            .await
            .map_err(GitHubRepoError::JsonParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_options_default() {
        let opts = SyncOptions::default();
        assert_eq!(opts.page, 1);
    }
}
