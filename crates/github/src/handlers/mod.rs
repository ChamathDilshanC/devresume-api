pub mod issue;
pub mod pull_request;
pub mod push;
pub mod release;
pub mod repository;
pub mod workflow;

pub use issue::{handle_issue_event, IssueEventPayload};
pub use pull_request::{handle_pull_request_event, PullRequestEventPayload};
pub use push::{handle_push_event, PushEventPayload};
pub use release::{handle_release_event, ReleaseEventPayload};
pub use repository::{handle_repository_event, RepositoryEventPayload};
pub use workflow::{handle_workflow_run_event, WorkflowRunEventPayload};

pub fn process_github_webhook_event(event_type: &str, raw_payload: &str) -> Result<String, String> {
    match event_type {
        "push" => {
            let payload: PushEventPayload = serde_json::from_str(raw_payload)
                .map_err(|e| format!("Failed to parse push event: {e}"))?;
            Ok(handle_push_event(&payload))
        }
        "pull_request" => {
            let payload: PullRequestEventPayload = serde_json::from_str(raw_payload)
                .map_err(|e| format!("Failed to parse pull_request event: {e}"))?;
            Ok(handle_pull_request_event(&payload))
        }
        "release" => {
            let payload: ReleaseEventPayload = serde_json::from_str(raw_payload)
                .map_err(|e| format!("Failed to parse release event: {e}"))?;
            Ok(handle_release_event(&payload))
        }
        "issues" | "issue_comment" => {
            let payload: IssueEventPayload = serde_json::from_str(raw_payload)
                .map_err(|e| format!("Failed to parse issue event: {e}"))?;
            Ok(handle_issue_event(&payload))
        }
        "workflow_run" => {
            let payload: WorkflowRunEventPayload = serde_json::from_str(raw_payload)
                .map_err(|e| format!("Failed to parse workflow_run event: {e}"))?;
            Ok(handle_workflow_run_event(&payload))
        }
        "repository" | "create" | "delete" | "fork" => {
            let payload: RepositoryEventPayload = serde_json::from_str(raw_payload)
                .map_err(|e| format!("Failed to parse repository event: {e}"))?;
            Ok(handle_repository_event(&payload))
        }
        unhandled => Ok(format!(
            "Ignored unsupported webhook event type: {unhandled}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_unhandled_event() {
        let res = process_github_webhook_event("ping", "{}").unwrap();
        assert!(res.contains("Ignored unsupported"));
    }
}
