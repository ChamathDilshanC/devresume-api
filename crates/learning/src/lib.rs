use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LearningModule {
    pub title: String,
    pub skill: String,
    pub estimated_hours: i32,
}

pub fn get_recommended_learning_path(skill: &str) -> Vec<LearningModule> {
    vec![LearningModule {
        title: format!("Mastering {} for High-Performance Backends", skill),
        skill: skill.to_string(),
        estimated_hours: 15,
    }]
}
