pub fn compute_rrf_score(keyword_rank: usize, vector_rank: usize, k: usize) -> f64 {
    (1.0 / (k + keyword_rank) as f64) + (1.0 / (k + vector_rank) as f64)
}
