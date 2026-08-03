use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerInsight {
    pub current_trajectory: String,
    pub recommended_skill_focus: Vec<String>,
}

pub fn generate_career_insights() -> CareerInsight {
    CareerInsight {
        current_trajectory: "Senior Backend / Systems Engineer".to_string(),
        recommended_skill_focus: vec!["Kubernetes".to_string(), "gRPC".to_string()],
    }
}
