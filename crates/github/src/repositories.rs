use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

pub struct GithubRepoClient {
    client: Client,
}

impl GithubRepoClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder().user_agent("DevResume-AI").build().unwrap(),
        }
    }

    pub async fn fetch_repositories(&self, token: &str) -> anyhow::Result<Vec<GithubRepo>> {
        let response = self
            .client
            .get("https://api.github.com/user/repos")
            .bearer_auth(token)
            .send()
            .await?;
        let repos = response.json::<Vec<GithubRepo>>().await?;
        Ok(repos)
    }
}
