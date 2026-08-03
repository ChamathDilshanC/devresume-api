use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    pub total_repositories: i32,
    pub total_commits: i32,
    pub primary_language: String,
    pub total_impact_score: f64,
}

pub fn calculate_developer_metrics(repo_count: i32, commit_count: i32) -> AnalyticsSummary {
    AnalyticsSummary {
        total_repositories: repo_count,
        total_commits: commit_count,
        primary_language: "Rust".to_string(),
        total_impact_score: (commit_count as f64) * 1.5,
    }
}
