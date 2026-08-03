pub fn calculate_developer_percentile(total_score: f64) -> f64 {
    if total_score > 500.0 {
        98.5
    } else {
        85.0
    }
}
