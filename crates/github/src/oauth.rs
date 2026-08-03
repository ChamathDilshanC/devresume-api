pub fn build_github_oauth_url(client_id: &str, redirect_uri: &str) -> String {
    format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user,repo",
        client_id, redirect_uri
    )
}
