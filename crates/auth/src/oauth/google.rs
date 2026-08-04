use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserProfile {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
}

pub struct GoogleOAuthClient {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub fn get_authorization_url(&self, state: &str) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email%20profile&state={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(state)
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<GoogleTokenResponse, String> {
        let client = reqwest::Client::new();
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(text);
        }

        let token_resp = response
            .json::<GoogleTokenResponse>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(token_resp)
    }

    pub async fn get_user_profile(&self, access_token: &str) -> Result<GoogleUserProfile, String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(text);
        }

        let profile = response
            .json::<GoogleUserProfile>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(profile)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_auth_url() {
        let client = GoogleOAuthClient::new(
            "g_client".to_string(),
            "g_secret".to_string(),
            "http://localhost:8080/google/callback".to_string(),
        );

        let url = client.get_authorization_url("state_123");
        assert!(url.contains("client_id=g_client"));
        assert!(url.contains("state=state_123"));
    }
}
