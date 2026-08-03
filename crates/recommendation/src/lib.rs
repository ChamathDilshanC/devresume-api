use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerRecommendation {
    pub title: String,
    pub description: String,
    pub action_url: String,
}

pub fn generate_recommendation_insights() -> Vec<CareerRecommendation> {
    vec![CareerRecommendation {
        title: "Add Microservice Architecture Project".to_string(),
        description: "Your profile is strong in SQLx and Axum. Building a distributed event pipeline will boost your senior impact score by 25%.".to_string(),
        action_url: "/repositories/new".to_string(),
    }]
}
