pub fn build_resume_prompt(repo_name: &str, tech_stack: &[&str]) -> String {
    format!(
        "Generate bullet points for developer resume project '{}' utilizing technologies: {:?}",
        repo_name, tech_stack
    )
}
