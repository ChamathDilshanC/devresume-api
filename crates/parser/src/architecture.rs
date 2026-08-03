use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ArchitecturePattern {
    ModularMonolith,
    Microservices,
    Serverless,
    Monolith,
}

pub fn detect_architecture_pattern(filenames: &[&str]) -> ArchitecturePattern {
    let mut has_docker_compose = false;
    let mut has_kubernetes = false;
    let mut sub_crate_count = 0;

    for f in filenames {
        if f.contains("docker-compose") {
            has_docker_compose = true;
        }
        if f.contains("k8s") || f.contains("kubernetes") || f.contains("helm") {
            has_kubernetes = true;
        }
        if (f.contains("Cargo.toml") || f.contains("package.json"))
            && (f.contains("crates/") || f.contains("packages/") || f.contains("apps/"))
        {
            sub_crate_count += 1;
        }
    }

    if sub_crate_count >= 3 && !has_kubernetes {
        ArchitecturePattern::ModularMonolith
    } else if has_kubernetes || (has_docker_compose && sub_crate_count >= 5) {
        ArchitecturePattern::Microservices
    } else if filenames
        .iter()
        .any(|f| f.contains("serverless") || f.contains("lambda"))
    {
        ArchitecturePattern::Serverless
    } else {
        ArchitecturePattern::Monolith
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_monolith_detection() {
        let files = vec![
            "Cargo.toml",
            "crates/auth/Cargo.toml",
            "crates/github/Cargo.toml",
            "crates/parser/Cargo.toml",
        ];

        let pattern = detect_architecture_pattern(&files);
        assert_eq!(pattern, ArchitecturePattern::ModularMonolith);
    }
}
