use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RequirementsTxtAnalysis {
    pub packages: Vec<String>,
}

pub fn parse_requirements_txt(content: &str) -> RequirementsTxtAnalysis {
    let mut packages = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('-') {
            continue;
        }

        // Split on operators ==, >=, <=, ~=, >
        let name = trimmed
            .split(&['=', '>', '<', '~', ';'][..])
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        if !name.is_empty() {
            packages.push(name);
        }
    }

    RequirementsTxtAnalysis { packages }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_requirements_txt() {
        let content = r#"
# Core packages
fastapi==0.100.0
pydantic>=2.0
numpy
"#;

        let analysis = parse_requirements_txt(content);
        assert!(analysis.packages.contains(&"fastapi".to_string()));
        assert!(analysis.packages.contains(&"pydantic".to_string()));
        assert!(analysis.packages.contains(&"numpy".to_string()));
    }
}
