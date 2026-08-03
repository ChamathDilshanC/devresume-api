use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AtsScoreResult {
    pub overall_score: i32,
    pub keyword_matches: Vec<String>,
    pub formatting_score: i32,
    pub suggestions: Vec<String>,
}

pub fn analyze_resume_ats(resume_text: &str) -> AtsScoreResult {
    let mut score = 85;
    let mut suggestions = vec![];

    if !resume_text.to_lowercase().contains("rust") {
        suggestions.push("Consider highlighting systems programming skills like Rust.".to_string());
    } else {
        score += 10;
    }

    AtsScoreResult {
        overall_score: score.min(100),
        keyword_matches: vec![
            "Rust".to_string(),
            "PostgreSQL".to_string(),
            "API".to_string(),
        ],
        formatting_score: 95,
        suggestions,
    }
}
