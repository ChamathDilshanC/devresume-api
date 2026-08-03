pub mod career_insights;
pub mod heatmap;
pub mod score_trends;
pub mod tech_distribution;
pub mod timeline;

pub use career_insights::{generate_career_insights, CareerInsight};
pub use heatmap::{generate_activity_heatmap, ActivityHeatmap, HeatmapDay};
pub use score_trends::{track_score_trends, ScorePoint, ScoreTrendTracker};
pub use tech_distribution::{
    analyze_technology_distribution, LanguagePercentage, TechnologyDistribution,
};
pub use timeline::{generate_career_timeline, CareerMilestone, CareerTimeline, MilestoneCategory};
