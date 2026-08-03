use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthSessionResult {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub avatar_url: String,
    pub access_token: String,
    pub refresh_token: String,
}
