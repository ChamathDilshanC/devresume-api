pub fn compute_activity_score(days_active: i32) -> f64 {
    (days_active as f64 / 30.0) * 100.0
}
