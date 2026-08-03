use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OAuthState {
    pub token: String,
    pub created_at: i64,
}

impl OAuthState {
    pub fn generate() -> Self {
        Self {
            token: format!("st_{}", Uuid::new_v4().simple()),
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn verify(&self, input_state: &str, max_age_seconds: i64) -> bool {
        if self.token != input_state {
            return false;
        }

        let now = chrono::Utc::now().timestamp();
        (now - self.created_at) <= max_age_seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_state_verification() {
        let state = OAuthState::generate();
        assert!(state.verify(&state.token, 300));
        assert!(!state.verify("invalid_state", 300));
    }
}
