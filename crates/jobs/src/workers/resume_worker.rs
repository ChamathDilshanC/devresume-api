use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumeJobPayload {
    pub user_id: String,
    pub target_format: String, // "json", "pdf", "docx", "markdown", "html"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumeJobResult {
    pub user_id: String,
    pub format: String,
    pub export_url: String,
    pub status: String,
}

pub struct ResumeWorker;

impl ResumeWorker {
    pub async fn process_resume_job(payload: &ResumeJobPayload) -> Result<ResumeJobResult, String> {
        Ok(ResumeJobResult {
            user_id: payload.user_id.clone(),
            format: payload.target_format.clone(),
            export_url: format!(
                "/exports/resumes/{}.{}",
                payload.user_id, payload.target_format
            ),
            status: "completed".to_string(),
        })
    }
}
