use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PackageJsonRaw {
    pub name: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PackageJsonAnalysis {
    pub name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
}

pub fn parse_package_json(content: &str) -> PackageJsonAnalysis {
    let raw: Result<PackageJsonRaw, _> = serde_json::from_str(content);
    match raw {
        Ok(data) => PackageJsonAnalysis {
            name: data.name,
            dependencies: data
                .dependencies
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default(),
            dev_dependencies: data
                .dev_dependencies
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default(),
        },
        Err(_) => PackageJsonAnalysis::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_json() {
        let json = r#"{
            "name": "devresume-web",
            "dependencies": {
                "next": "^14.0.0",
                "react": "^18.2.0"
            },
            "devDependencies": {
                "typescript": "^5.0.0"
            }
        }"#;

        let analysis = parse_package_json(json);
        assert_eq!(analysis.name, Some("devresume-web".to_string()));
        assert!(analysis.dependencies.contains(&"next".to_string()));
        assert!(analysis
            .dev_dependencies
            .contains(&"typescript".to_string()));
    }
}
