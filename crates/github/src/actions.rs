pub fn detect_github_actions_workflows(files: &[&str]) -> bool {
    files.iter().any(|f| f.contains(".github/workflows"))
}
