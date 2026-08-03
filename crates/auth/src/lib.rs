pub mod api_keys;
pub mod jwt;
pub mod oauth;
pub mod permissions;
pub mod rbac;
pub mod refresh_tokens;

pub use api_keys::generate_api_key;
pub use jwt::{create_jwt, verify_jwt, Claims};
pub use oauth::validate_oauth_provider;
pub use permissions::check_permission;
pub use rbac::{has_role, Role};
pub use refresh_tokens::generate_refresh_token;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}
