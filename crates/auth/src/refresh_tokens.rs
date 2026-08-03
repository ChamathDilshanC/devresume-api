use uuid::Uuid;

pub fn generate_refresh_token() -> String {
    format!("rt_{}", Uuid::new_v4().simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_refresh_token_format() {
        let token = generate_refresh_token();
        assert!(token.starts_with("rt_"));
        assert_eq!(token.len(), 35); // "rt_" (3) + 32 hex chars
    }

    #[test]
    fn test_generate_refresh_tokens_are_unique() {
        let token1 = generate_refresh_token();
        let token2 = generate_refresh_token();
        assert_ne!(token1, token2);
    }
}
