use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IssueEventPayload {
    pub action: String, // "opened", "closed", "labeled"
    pub issue: IssueDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IssueDetails {
    pub number: u64,
    pub title: String,
    pub state: String,
}

pub fn handle_issue_event(payload: &IssueEventPayload) -> String {
    format!(
        "Processed issue event #{}: action={}, title='{}'",
        payload.issue.number, payload.action, payload.issue.title
    )
}
