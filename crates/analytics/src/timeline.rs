use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum MilestoneCategory {
    LanguageLearned,
    OpenSourceContribution,
    InfrastructureDeployed,
    AiModelIntegrated,
    ResumePublished,
    PortfolioReleased,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CareerMilestone {
    pub year: u32,
    pub date: String,
    pub title: String,
    pub category: MilestoneCategory,
    pub description: String,
    pub repo_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CareerTimeline {
    pub developer_name: String,
    pub start_year: u32,
    pub total_milestones: usize,
    pub milestones: Vec<CareerMilestone>,
}

pub fn generate_career_timeline(dev_name: &str) -> CareerTimeline {
    let milestones = vec![
        CareerMilestone {
            year: 2024,
            date: "2024-03-15".to_string(),
            title: "Started Systems Programming in Rust".to_string(),
            category: MilestoneCategory::LanguageLearned,
            description: "Mastered memory safety, ownership, and concurrency in Rust.".to_string(),
            repo_name: None,
        },
        CareerMilestone {
            year: 2024,
            date: "2024-08-10".to_string(),
            title: "First Open Source Project".to_string(),
            category: MilestoneCategory::OpenSourceContribution,
            description: "Published modular Rust crates for distributed systems.".to_string(),
            repo_name: Some("devresume-api".to_string()),
        },
        CareerMilestone {
            year: 2025,
            date: "2025-01-20".to_string(),
            title: "First Docker & PostgreSQL Deployment".to_string(),
            category: MilestoneCategory::InfrastructureDeployed,
            description: "Built multi-stage Docker containers with pgvector storage.".to_string(),
            repo_name: Some("devresume-api".to_string()),
        },
        CareerMilestone {
            year: 2025,
            date: "2025-06-12".to_string(),
            title: "AI Embeddings & Multi-LLM Pipeline".to_string(),
            category: MilestoneCategory::AiModelIntegrated,
            description: "Integrated OpenAI, Gemini, Claude, and Ollama with Hybrid RRF search."
                .to_string(),
            repo_name: Some("devresume-api".to_string()),
        },
        CareerMilestone {
            year: 2026,
            date: "2026-08-01".to_string(),
            title: "Published Resume Version v1.0".to_string(),
            category: MilestoneCategory::ResumePublished,
            description: "Generated ATS-optimized developer resume withSTAR impact bullet points."
                .to_string(),
            repo_name: None,
        },
        CareerMilestone {
            year: 2026,
            date: "2026-08-03".to_string(),
            title: "Released Developer Portfolio Website".to_string(),
            category: MilestoneCategory::PortfolioReleased,
            description: "Deployed Glassmorphism developer portfolio with live SEO schema."
                .to_string(),
            repo_name: None,
        },
    ];

    CareerTimeline {
        developer_name: dev_name.to_string(),
        start_year: 2024,
        total_milestones: milestones.len(),
        milestones,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_career_timeline_generation() {
        let timeline = generate_career_timeline("Chamath Dilshan");
        assert_eq!(timeline.start_year, 2024);
        assert_eq!(timeline.total_milestones, 6);
        assert_eq!(timeline.milestones[0].year, 2024);
        assert_eq!(timeline.milestones[5].year, 2026);
    }
}
