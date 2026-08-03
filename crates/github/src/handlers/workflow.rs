use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkflowRunEventPayload {
    pub action: String, // "completed", "requested"
    pub workflow_run: WorkflowRunDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkflowRunDetails {
    pub id: i64,
    pub name: Option<String>,
    pub status: String,
    pub conclusion: Option<String>,
}

pub fn handle_workflow_run_event(payload: &WorkflowRunEventPayload) -> String {
    format!(
        "Processed workflow_run event #{}: status={}, conclusion={:?}",
        payload.workflow_run.id, payload.workflow_run.status, payload.workflow_run.conclusion
    )
}
