use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ReadmeAnalysis {
    pub title: Option<String>,
    pub summary: String,
    pub detected_badges: Vec<String>,
}

pub fn parse_readme(content: &str) -> ReadmeAnalysis {
    let mut title = None;
    let mut summary_lines = Vec::new();
    let mut badges = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("# ") && title.is_none() {
            title = Some(trimmed.trim_start_matches("# ").to_string());
        } else if trimmed.contains("![") && trimmed.contains("](") {
            badges.push(trimmed.to_string());
        } else if !trimmed.is_empty() && !trimmed.starts_with('#') && summary_lines.len() < 3 {
            summary_lines.push(trimmed.to_string());
        }
    }

    ReadmeAnalysis {
        title,
        summary: summary_lines.join(" "),
        detected_badges: badges,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_readme() {
        let content = r#"
# DevResume AI

[![CI](https://github.com/badge.svg)](https://github.com)

DevResume AI is an enterprise platform for developers.
It automatically extracts technical profiles from repositories.
"#;

        let analysis = parse_readme(content);
        assert_eq!(analysis.title, Some("DevResume AI".to_string()));
        assert!(analysis.summary.contains("enterprise platform"));
        assert_eq!(analysis.detected_badges.len(), 1);
    }
}
