pub struct ResumeSummaryAgent;

impl ResumeSummaryAgent {
    pub fn summarize_experience(experience_raw: &str) -> String {
        format!("Refined Experience: {}", experience_raw)
    }
}
