pub fn compute_contribution_score(commits: i32, pull_requests: i32) -> f64 {
    (commits as f64 * 1.5) + (pull_requests as f64 * 5.0)
}
