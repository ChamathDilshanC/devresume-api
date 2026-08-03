use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub title: String,
    pub message: String,
    pub recipient: String,
}

pub fn send_notification(payload: NotificationPayload) -> bool {
    tracing::info!("Sending notification to {}: {}", payload.recipient, payload.title);
    true
}
