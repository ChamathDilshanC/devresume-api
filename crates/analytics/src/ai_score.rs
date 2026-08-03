pub fn compute_ai_code_quality_score(complexity_index: f64) -> i32 {
    let score = 100.0 - complexity_index;
    score.clamp(0.0, 100.0) as i32
}
