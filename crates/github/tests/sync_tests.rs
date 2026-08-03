use github::{calculate_commit_impact, GithubRepo, SyncOptions};

#[test]
fn test_github_repo_struct_serialization() {
    let repo = GithubRepo {
        id: 123456,
        name: "DevResume-AI".to_string(),
        full_name: "ChamathDilshanC/DevResume-AI".to_string(),
        html_url: "https://github.com/ChamathDilshanC/DevResume-AI".to_string(),
        description: Some("AI-powered resume platform".to_string()),
        default_branch: "main".to_string(),
        private: false,
        language: Some("Rust".to_string()),
        stargazers_count: 42,
        forks_count: 10,
        open_issues_count: 2,
        updated_at: Some("2026-08-03T00:00:00Z".to_string()),
    };

    let json = serde_json::to_string(&repo).expect("Serialization failed");
    assert!(json.contains("DevResume-AI"));

    let deserialized: GithubRepo = serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(deserialized.id, 123456);
}

#[test]
fn test_commit_impact_calculation() {
    assert_eq!(calculate_commit_impact(10, 10), 2.0);
    assert_eq!(calculate_commit_impact(500, 500), 100.0);
}

#[test]
fn test_sync_options_construction() {
    let opts = SyncOptions {
        page: 2,
        per_page: 50,
        visibility: "public".to_string(),
        sort: "pushed".to_string(),
    };

    assert_eq!(opts.page, 2);
    assert_eq!(opts.per_page, 50);
}
