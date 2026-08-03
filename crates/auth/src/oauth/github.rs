use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubOAuthError {
    #[error("Failed to build HTTP client: {0}")]
    ClientBuild(reqwest::Error),
    #[error("Token exchange failed: {0}")]
    TokenExchange(reqwest::Error),
    #[error("User fetch failed: {0}")]
    UserFetch(reqwest::Error),
    #[error("GitHub API returned error: {0}")]
    ApiError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUserProfile {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub html_url: String,
}

pub struct GitHubOAuthClient {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl GitHubOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub fn get_authorization_url(&self, state: &str) -> String {
        format!(
            "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email,read:user&state={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(state)
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<GitHubTokenResponse, GitHubOAuthError> {
        let client = reqwest::Client::builder()
            .user_agent("DevResume-AI")
            .build()
            .map_err(GitHubOAuthError::ClientBuild)?;

        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        let response = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await
            .map_err(GitHubOAuthError::TokenExchange)?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(GitHubOAuthError::ApiError(text));
        }

        let token_resp = response
            .json::<GitHubTokenResponse>()
            .await
            .map_err(GitHubOAuthError::TokenExchange)?;

        Ok(token_resp)
    }

    pub async fn get_user_profile(
        &self,
        access_token: &str,
    ) -> Result<GitHubUserProfile, GitHubOAuthError> {
        let client = reqwest::Client::builder()
            .user_agent("DevResume-AI")
            .build()
            .map_err(GitHubOAuthError::ClientBuild)?;

        let response = client
            .get("https://api.github.com/user")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(GitHubOAuthError::UserFetch)?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(GitHubOAuthError::ApiError(text));
        }

        let profile = response
            .json::<GitHubUserProfile>()
            .await
            .map_err(GitHubOAuthError::UserFetch)?;

        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_url_generation() {
        let client = GitHubOAuthClient::new(
            "client_123".to_string(),
            "secret_456".to_string(),
            "http://localhost:8080/callback".to_string(),
        );

        let url = client.get_authorization_url("state_abc");
        assert!(url.contains("client_id=client_123"));
        assert!(url.contains("state=state_abc"));
        assert!(url.contains("user:email"));
    }
}
