use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullRequestEventPayload {
    pub action: String, // "opened", "closed", "synchronize", "reopened"
    pub number: u64,
    pub pull_request: PullRequestDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullRequestDetails {
    pub id: i64,
    pub title: String,
    pub state: String,
    pub merged: Option<bool>,
    pub additions: Option<i32>,
    pub deletions: Option<i32>,
}

pub fn handle_pull_request_event(payload: &PullRequestEventPayload) -> String {
    format!(
        "Processed pull_request event #{}: action={}, state={}",
        payload.number, payload.action, payload.pull_request.state
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pull_request_handler() {
        let payload = PullRequestEventPayload {
            action: "opened".to_string(),
            number: 42,
            pull_request: PullRequestDetails {
                id: 100,
                title: "feat: PR Title".to_string(),
                state: "open".to_string(),
                merged: Some(false),
                additions: Some(100),
                deletions: Some(20),
            },
        };

        let res = handle_pull_request_event(&payload);
        assert!(res.contains("#42"));
        assert!(res.contains("opened"));
    }
}
