pub mod callback;
pub mod github;
pub mod google;
pub mod state;

pub use callback::{OAuthCallbackQuery, OAuthSessionResult};
pub use github::{GitHubOAuthClient, GitHubOAuthError, GitHubUserProfile};
pub use google::{GoogleOAuthClient, GoogleUserProfile};
pub use state::OAuthState;

pub fn validate_oauth_provider(provider: &str) -> bool {
    matches!(provider, "github" | "google")
}
