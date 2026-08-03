use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitStats {
    pub hash: String,
    pub author_name: String,
    pub message: String,
    pub additions: i32,
    pub deletions: i32,
}

pub fn calculate_commit_impact(additions: i32, deletions: i32) -> f64 {
    (additions + deletions) as f64 * 0.1
}
