use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_jwt(
    user_id: Uuid,
    email: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    create_jwt_with_ttl(user_id, email, secret, Duration::days(7))
}

pub fn create_jwt_with_ttl(
    user_id: Uuid,
    email: &str,
    secret: &str,
    ttl: Duration,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expiration = now
        .checked_add_signed(ttl)
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_jwt() {
        let user_id = Uuid::new_v4();
        let email = "dev@devresume.ai";
        let secret = "super_secret_jwt_key_for_unit_tests";

        let token = create_jwt(user_id, email, secret).expect("Token generation should succeed");
        assert!(!token.is_empty());

        let claims = verify_jwt(&token, secret).expect("Token verification should succeed");
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_invalid_secret_fails_verification() {
        let user_id = Uuid::new_v4();
        let email = "dev@devresume.ai";
        let secret = "correct_secret";
        let wrong_secret = "wrong_secret";

        let token = create_jwt(user_id, email, secret).expect("Token generation should succeed");
        let result = verify_jwt(&token, wrong_secret);

        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_token_fails_verification() {
        let secret = "secret";
        assert!(verify_jwt("not_a_valid_jwt_token", secret).is_err());
        assert!(verify_jwt("header.payload", secret).is_err());
        assert!(verify_jwt("", secret).is_err());
    }

    #[test]
    fn test_expired_token_fails_verification() {
        let user_id = Uuid::new_v4();
        let email = "dev@devresume.ai";
        let secret = "secret";

        // jsonwebtoken has a default 60-second leeway, so set -120 seconds to guarantee expiration
        let token = create_jwt_with_ttl(user_id, email, secret, Duration::seconds(-120))
            .expect("Token generation should succeed");

        let result = verify_jwt(&token, secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_token_fails_verification() {
        let user_id = Uuid::new_v4();
        let email = "dev@devresume.ai";
        let secret = "super_secret_key";

        let token = create_jwt(user_id, email, secret).expect("Token generation should succeed");
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        // Tamper with payload
        let tampered_payload = format!("{}.eyJzdWIiOiJoYWNrZXIifQ.{}", parts[0], parts[2]);
        let result = verify_jwt(&tampered_payload, secret);
        assert!(result.is_err());
    }
}
