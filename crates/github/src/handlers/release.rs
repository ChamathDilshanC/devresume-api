use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseEventPayload {
    pub action: String, // "published", "created"
    pub release: ReleaseDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseDetails {
    pub tag_name: String,
    pub name: Option<String>,
    pub prerelease: bool,
}

pub fn handle_release_event(payload: &ReleaseEventPayload) -> String {
    format!(
        "Processed release event: tag={}, action={}",
        payload.release.tag_name, payload.action
    )
}
