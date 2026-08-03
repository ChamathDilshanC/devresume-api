use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioConfig {
    pub title: String,
    pub bio: String,
    pub theme: String,
    pub featured_projects: Vec<String>,
}

pub fn generate_default_portfolio(user_name: &str) -> PortfolioConfig {
    PortfolioConfig {
        title: format!("{}'s Engineering Portfolio", user_name),
        bio: "Building high-performance software applications and distributed systems.".to_string(),
        theme: "dark".to_string(),
        featured_projects: vec![],
    }
}
