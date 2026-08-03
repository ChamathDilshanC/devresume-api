use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DockerfileAnalysis {
    pub base_images: Vec<String>,
    pub exposed_ports: Vec<u16>,
    pub is_multistage: bool,
}

pub fn parse_dockerfile(content: &str) -> DockerfileAnalysis {
    let mut analysis = DockerfileAnalysis::default();

    for line in content.lines() {
        let trimmed = line.trim();
        let upper = trimmed.to_uppercase();

        if upper.starts_with("FROM ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                analysis.base_images.push(parts[1].to_string());
            }
        } else if upper.starts_with("EXPOSE ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            for part in parts.iter().skip(1) {
                if let Ok(port) = part.parse::<u16>() {
                    analysis.exposed_ports.push(port);
                }
            }
        }
    }

    analysis.is_multistage = analysis.base_images.len() > 1;
    analysis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dockerfile() {
        let content = r#"
FROM rust:1.78 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
EXPOSE 8080 9090
"#;

        let analysis = parse_dockerfile(content);
        assert_eq!(analysis.base_images.len(), 2);
        assert!(analysis.is_multistage);
        assert_eq!(analysis.exposed_ports, vec![8080, 9090]);
    }
}
