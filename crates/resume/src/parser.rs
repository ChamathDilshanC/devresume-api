use crate::builder::ResumeData;

pub fn parse_resume_text(raw_text: &str) -> anyhow::Result<ResumeData> {
    Ok(ResumeData {
        name: "Parsed Developer".to_string(),
        email: "developer@example.com".to_string(),
        title: "Software Engineer".to_string(),
        summary: raw_text.to_string(),
        skills: vec!["Rust".to_string(), "Python".to_string()],
        projects: vec![],
    })
}
