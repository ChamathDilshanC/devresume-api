pub mod api_keys;
pub mod jwt;
pub mod middleware;
pub mod oauth;
pub mod passwords;
pub mod permissions;
pub mod rbac;
pub mod refresh_tokens;

pub use api_keys::generate_api_key;
pub use jwt::{create_jwt, create_jwt_with_ttl, verify_jwt, Claims};
pub use middleware::{AuthError, AuthUser};
pub use oauth::{
    validate_oauth_provider, GitHubOAuthClient, GitHubOAuthError, GitHubUserProfile,
    GoogleOAuthClient, GoogleUserProfile, OAuthCallbackQuery, OAuthSessionResult, OAuthState,
};
pub use passwords::{hash_password, verify_password};
pub use permissions::{
    check_permission, get_permissions_for_role, role_has_permission, Permission,
};
pub use rbac::{has_role, Role};
pub use refresh_tokens::generate_refresh_token;
