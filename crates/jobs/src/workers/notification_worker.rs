use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationJobPayload {
    pub recipient_email: String,
    pub subject: String,
    pub body: String,
}

pub struct NotificationWorker;

impl NotificationWorker {
    pub async fn process_notification(payload: &NotificationJobPayload) -> Result<String, String> {
        if payload.recipient_email.is_empty() {
            return Err("Recipient email required".to_string());
        }
        Ok(format!("Notification sent to {}", payload.recipient_email))
    }
}
