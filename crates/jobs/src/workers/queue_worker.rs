use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JobType {
    Sync,
    Embedding,
    AI,
    Resume,
    Portfolio,
    Notification,
    Cleanup,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: String,
    pub job_type: JobType,
    pub payload_json: String,
    pub retry_count: usize,
    pub max_retries: usize,
    pub status: String, // "queued", "processing", "completed", "failed", "dead_letter"
    pub error_message: Option<String>,
}

#[derive(Debug, Default)]
pub struct JobQueueEngine {
    pub dead_letter_queue: Vec<Job>,
}

impl JobQueueEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_job_failure(&mut self, mut job: Job, err: &str) {
        job.retry_count += 1;
        job.error_message = Some(err.to_string());

        if job.retry_count >= job.max_retries {
            job.status = "dead_letter".to_string();
            self.dead_letter_queue.push(job);
        } else {
            job.status = "queued".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_letter_queue_handling() {
        let mut queue = JobQueueEngine::new();
        let job = Job {
            id: "job-1".to_string(),
            job_type: JobType::Sync,
            payload_json: "{}".to_string(),
            retry_count: 2,
            max_retries: 3,
            status: "processing".to_string(),
            error_message: None,
        };

        queue.handle_job_failure(job, "Rate limit exceeded");
        assert_eq!(queue.dead_letter_queue.len(), 1);
        assert_eq!(queue.dead_letter_queue[0].status, "dead_letter");
    }
}
