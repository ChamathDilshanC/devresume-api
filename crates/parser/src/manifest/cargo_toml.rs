use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CargoTomlAnalysis {
    pub package_name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub is_workspace: bool,
}

pub fn parse_cargo_toml(content: &str) -> CargoTomlAnalysis {
    let mut analysis = CargoTomlAnalysis::default();

    if content.contains("[workspace]") {
        analysis.is_workspace = true;
    }

    let mut in_deps = false;
    let mut in_dev_deps = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if (trimmed.starts_with("name ") || trimmed.starts_with("name="))
            && analysis.package_name.is_none()
        {
            let name = trimmed
                .split('=')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string());
            analysis.package_name = name;
        } else if trimmed.starts_with("[dependencies]") {
            in_deps = true;
            in_dev_deps = false;
        } else if trimmed.starts_with("[dev-dependencies]") {
            in_deps = false;
            in_dev_deps = true;
        } else if trimmed.starts_with('[') {
            in_deps = false;
            in_dev_deps = false;
        } else if (in_deps || in_dev_deps) && trimmed.contains('=') && !trimmed.starts_with('#') {
            let dep_name = trimmed.split('=').next().unwrap_or("").trim().to_string();
            if !dep_name.is_empty() {
                if in_deps {
                    analysis.dependencies.push(dep_name);
                } else {
                    analysis.dev_dependencies.push(dep_name);
                }
            }
        }
    }

    analysis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_toml() {
        let content = r#"
[package]
name = "devresume-api"

[dependencies]
axum = "0.7"
sqlx = "0.7"
tokio = "1.0"

[dev-dependencies]
reqwest = "0.12"
"#;

        let analysis = parse_cargo_toml(content);
        assert_eq!(analysis.package_name, Some("devresume-api".to_string()));
        assert!(analysis.dependencies.contains(&"axum".to_string()));
        assert!(analysis.dependencies.contains(&"sqlx".to_string()));
        assert!(analysis.dev_dependencies.contains(&"reqwest".to_string()));
    }
}
