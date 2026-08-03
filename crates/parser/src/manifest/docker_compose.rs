use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DockerComposeAnalysis {
    pub services: Vec<String>,
    pub detected_databases: Vec<String>,
}

pub fn parse_docker_compose(content: &str) -> DockerComposeAnalysis {
    let mut analysis = DockerComposeAnalysis::default();

    let mut in_services = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("services:") {
            in_services = true;
            continue;
        }

        if in_services && !trimmed.starts_with('#') && !trimmed.is_empty() {
            // Check 2-space indented service key
            if line.starts_with("  ") && !line.starts_with("    ") && line.ends_with(':') {
                let service_name = trimmed.trim_end_matches(':').to_string();
                if !service_name.is_empty() {
                    let lower = service_name.to_lowercase();
                    if lower.contains("postgres") {
                        analysis.detected_databases.push("PostgreSQL".to_string());
                    } else if lower.contains("redis") {
                        analysis.detected_databases.push("Redis".to_string());
                    } else if lower.contains("mongo") {
                        analysis.detected_databases.push("MongoDB".to_string());
                    } else if lower.contains("mysql") {
                        analysis.detected_databases.push("MySQL".to_string());
                    }
                    analysis.services.push(service_name);
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
    fn test_parse_docker_compose() {
        let content = r#"
version: '3.8'
services:
  postgres:
    image: postgres:16
  redis:
    image: redis:7
  app:
    build: .
"#;

        let analysis = parse_docker_compose(content);
        assert_eq!(analysis.services.len(), 3);
        assert!(analysis
            .detected_databases
            .contains(&"PostgreSQL".to_string()));
        assert!(analysis.detected_databases.contains(&"Redis".to_string()));
    }
}
