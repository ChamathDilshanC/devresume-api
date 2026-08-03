use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TechTrend {
    pub language: String,
    pub growth_percentage: f64,
}

pub fn analyze_technology_trends() -> Vec<TechTrend> {
    vec![TechTrend {
        language: "Rust".to_string(),
        growth_percentage: 45.2,
    }]
}
