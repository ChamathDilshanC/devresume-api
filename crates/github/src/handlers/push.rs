use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushEventPayload {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub before: String,
    pub after: String,
    pub repository: PushEventRepository,
    pub commits: Vec<PushEventCommit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushEventRepository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushEventCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
}

pub fn handle_push_event(payload: &PushEventPayload) -> String {
    format!(
        "Processed push event for repo {} with {} commits on ref {}",
        payload.repository.full_name,
        payload.commits.len(),
        payload.git_ref
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_event_handler() {
        let payload = PushEventPayload {
            git_ref: "refs/heads/main".to_string(),
            before: "000000".to_string(),
            after: "111111".to_string(),
            repository: PushEventRepository {
                id: 1,
                name: "repo".to_string(),
                full_name: "owner/repo".to_string(),
            },
            commits: vec![PushEventCommit {
                id: "111111".to_string(),
                message: "feat: add feature".to_string(),
                timestamp: "2026-08-03T00:00:00Z".to_string(),
            }],
        };

        let result = handle_push_event(&payload);
        assert!(result.contains("owner/repo"));
        assert!(result.contains("1 commits"));
    }
}
