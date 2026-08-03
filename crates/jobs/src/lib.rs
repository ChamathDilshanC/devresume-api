pub mod workers;

pub use workers::{
    AIJobPayload, AIJobResult, AIWorker, CleanupWorker, EmbeddingJobPayload, EmbeddingJobResult,
    EmbeddingWorker, Job, JobQueueEngine, JobType, NotificationJobPayload, NotificationWorker,
    PortfolioJobPayload, PortfolioJobResult, PortfolioWorker, ResumeJobPayload, ResumeJobResult,
    ResumeWorker, SyncJobPayload, SyncJobResult, SyncWorker,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: Uuid,
    pub company_name: String,
    pub job_title: String,
    pub ats_score: i32,
    pub status: String,
}

pub fn track_job_application(company: &str, title: &str) -> JobApplication {
    JobApplication {
        id: Uuid::new_v4(),
        company_name: company.to_string(),
        job_title: title.to_string(),
        ats_score: 88,
        status: "applied".to_string(),
    }
}
