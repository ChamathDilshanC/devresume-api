use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewQuestion {
    pub question: String,
    pub category: String,
    pub sample_answer_outline: Vec<String>,
}

pub fn generate_mock_interview_questions(topic: &str) -> Vec<InterviewQuestion> {
    vec![InterviewQuestion {
        question: format!("Explain how memory management and ownership work in {}", topic),
        category: "Systems Engineering".to_string(),
        sample_answer_outline: vec!["Ownership rules".to_string(), "Borrow checker".to_string(), "Lifetimes".to_string()],
    }]
}
