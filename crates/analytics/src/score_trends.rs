use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ScorePoint {
    pub timestamp: String,
    pub resume_version: String,
    pub ats_score: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ScoreTrendTracker {
    pub initial_score: u32,
    pub latest_score: u32,
    pub total_improvements: u32,
    pub trend_points: Vec<ScorePoint>,
}

pub fn track_score_trends(points: Vec<ScorePoint>) -> ScoreTrendTracker {
    let initial_score = points.first().map(|p| p.ats_score).unwrap_or(0);
    let latest_score = points.last().map(|p| p.ats_score).unwrap_or(0);
    let total_improvements = latest_score.saturating_sub(initial_score);

    ScoreTrendTracker {
        initial_score,
        latest_score,
        total_improvements,
        trend_points: points,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_trend_tracking() {
        let points = vec![
            ScorePoint {
                timestamp: "2026-08-01".to_string(),
                resume_version: "v1.0".to_string(),
                ats_score: 65,
            },
            ScorePoint {
                timestamp: "2026-08-03".to_string(),
                resume_version: "v1.5".to_string(),
                ats_score: 88,
            },
        ];

        let tracker = track_score_trends(points);
        assert_eq!(tracker.initial_score, 65);
        assert_eq!(tracker.latest_score, 88);
        assert_eq!(tracker.total_improvements, 23);
    }
}
