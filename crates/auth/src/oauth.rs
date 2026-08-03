pub fn validate_oauth_provider(provider: &str) -> bool {
    matches!(provider, "github" | "google" | "linkedin")
}
