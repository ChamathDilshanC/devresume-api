use crate::builder::{generate_resume_schema, ResumeSchema};

pub fn parse_resume_text(raw_text: &str) -> anyhow::Result<ResumeSchema> {
    Ok(generate_resume_schema(
        "Parsed Developer",
        "developer@example.com",
        "Software Engineer",
        raw_text,
        vec![],
        vec![],
    ))
}
