use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerGoal {
    pub id: Uuid,
    pub target_role: String,
    pub target_salary: Option<f64>,
    pub timeframe_months: i32,
    pub status: String,
}

pub fn create_career_goal(target_role: &str) -> CareerGoal {
    CareerGoal {
        id: Uuid::new_v4(),
        target_role: target_role.to_string(),
        target_salary: Some(120000.0),
        timeframe_months: 12,
        status: "active".to_string(),
    }
}
