use analytics::{
    analyze_technology_distribution, generate_activity_heatmap, generate_career_insights,
    generate_career_timeline, track_score_trends, ScorePoint,
};
use std::collections::HashMap;

#[test]
fn test_end_to_end_analytics_dashboard() {
    // 1. Heatmap & Commits
    let commit_dates = vec!["2026-08-01", "2026-08-02", "2026-08-03"];
    let heatmap = generate_activity_heatmap(&commit_dates);
    assert_eq!(heatmap.total_commits, 3);

    // 2. Language Distribution
    let mut lang_bytes = HashMap::new();
    lang_bytes.insert("Rust".to_string(), 7500);
    lang_bytes.insert("TypeScript".to_string(), 2500);
    let tech_dist = analyze_technology_distribution(&lang_bytes);
    assert_eq!(tech_dist.primary_stack, "Rust");

    // 3. Score Trends
    let points = vec![
        ScorePoint {
            timestamp: "2026-08-01".to_string(),
            resume_version: "v1.0".to_string(),
            ats_score: 72,
        },
        ScorePoint {
            timestamp: "2026-08-03".to_string(),
            resume_version: "v1.5".to_string(),
            ats_score: 92,
        },
    ];
    let score_tracker = track_score_trends(points);
    assert_eq!(score_tracker.total_improvements, 20);

    // 4. Career Timeline
    let timeline = generate_career_timeline("Chamath Dilshan");
    assert_eq!(timeline.start_year, 2024);
    assert_eq!(timeline.total_milestones, 6);

    // 5. Insights
    let insights = generate_career_insights();
    assert!(!insights.current_trajectory.is_empty());
}
