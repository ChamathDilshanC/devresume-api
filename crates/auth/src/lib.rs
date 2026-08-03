pub mod api_keys;
pub mod jwt;
pub mod oauth;
pub mod permissions;
pub mod rbac;
pub mod refresh_tokens;

pub use api_keys::generate_api_key;
pub use jwt::{create_jwt, create_jwt_with_ttl, verify_jwt, Claims};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "SecretPassword123!";
        let hash = hash_password(password).expect("Password hashing should succeed");
        assert_ne!(password, hash);

        let is_valid =
            verify_password(password, &hash).expect("Password verification should succeed");
        assert!(is_valid);

        let is_invalid =
            verify_password("WrongPassword", &hash).expect("Password verification should succeed");
        assert!(!is_invalid);
    }
}
