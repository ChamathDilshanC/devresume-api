use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectContext {
    pub name: String,
    pub description: Option<String>,
    pub primary_language: Option<String>,
    pub tech_stack: Vec<String>,
    pub readme_snippet: Option<String>,
}

pub fn build_project_summary_prompt(ctx: &ProjectContext) -> String {
    format!(
        "Analyze GitHub repository '{}' ({:?}). Primary language: {:?}. Technologies: {:?}. README: {:?}. Provide a concise 2-sentence technical summary in JSON format.",
        ctx.name,
        ctx.description.as_deref().unwrap_or("No description"),
        ctx.primary_language.as_deref().unwrap_or("Unknown"),
        ctx.tech_stack,
        ctx.readme_snippet.as_deref().unwrap_or("")
    )
}

pub fn build_resume_prompt(repo_name: &str, tech_stack: &[&str]) -> String {
    format!(
        "Generate 3 impact-driven STAR bullet points for a developer resume highlighting project '{}' built with {:?}.",
        repo_name, tech_stack
    )
}

pub fn build_ats_analysis_prompt(resume_text: &str, job_description: &str) -> String {
    format!(
        "Compare this developer resume against the job description for ATS score (0-100), missing keywords, and readability.\n\nRESUME:\n{}\n\nJOB DESCRIPTION:\n{}",
        resume_text, job_description
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_summary_prompt() {
        let ctx = ProjectContext {
            name: "DevResume-AI".to_string(),
            description: Some("AI platform".to_string()),
            primary_language: Some("Rust".to_string()),
            tech_stack: vec!["Axum".to_string(), "PostgreSQL".to_string()],
            readme_snippet: Some("README content".to_string()),
        };

        let prompt = build_project_summary_prompt(&ctx);
        assert!(prompt.contains("DevResume-AI"));
        assert!(prompt.contains("Rust"));
    }
}
