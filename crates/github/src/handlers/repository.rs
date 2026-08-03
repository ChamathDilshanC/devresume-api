use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryEventPayload {
    pub action: String, // "created", "deleted", "archived"
    pub repository: RepositoryEventDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryEventDetails {
    pub id: i64,
    pub name: String,
    pub full_name: String,
}

pub fn handle_repository_event(payload: &RepositoryEventPayload) -> String {
    format!(
        "Processed repository event: action={}, repo={}",
        payload.action, payload.repository.full_name
    )
}
