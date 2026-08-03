use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioJobPayload {
    pub user_id: String,
    pub template: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioJobResult {
    pub user_id: String,
    pub site_url: String,
    pub status: String,
}

pub struct PortfolioWorker;

impl PortfolioWorker {
    pub async fn process_portfolio_job(
        payload: &PortfolioJobPayload,
    ) -> Result<PortfolioJobResult, String> {
        Ok(PortfolioJobResult {
            user_id: payload.user_id.clone(),
            site_url: format!("https://portfolio.devresume.ai/{}", payload.user_id),
            status: "completed".to_string(),
        })
    }
}
