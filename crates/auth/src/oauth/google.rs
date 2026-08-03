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
