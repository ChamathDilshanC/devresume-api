use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash_str: &str) -> Result<bool, BcryptError> {
    verify(password, hash_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let raw = "SuperSecretPassword123!";
        let hashed = hash_password(raw).expect("Hashing failed");
        assert_ne!(raw, hashed);

        assert!(verify_password(raw, &hashed).unwrap());
        assert!(!verify_password("WrongPassword", &hashed).unwrap());
    }
}
