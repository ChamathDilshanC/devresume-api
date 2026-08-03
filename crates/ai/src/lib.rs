use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSummaryResult {
    pub title: String,
    pub bullet_points: Vec<String>,
    pub key_technologies: Vec<String>,
    pub achievements: Vec<String>,
}

pub struct AiClient {
    pub api_key: String,
}

impl AiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn generate_project_summary(&self, repo_name: &str, description: &str) -> anyhow::Result<ProjectSummaryResult> {
        // Fallback / standard AI prompt pipeline result
        Ok(ProjectSummaryResult {
            title: format!("Architected and built {}", repo_name),
            bullet_points: vec![
                format!("Developed scalable features for {}", repo_name),
                format!("Integrated modern software practices based on {}", description),
            ],
            key_technologies: vec!["Rust".to_string(), "PostgreSQL".to_string(), "Docker".to_string()],
            achievements: vec!["Optimized throughput and automated developer workflows".to_string()],
        })
    }
}
