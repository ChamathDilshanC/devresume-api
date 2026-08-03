use crate::manifest::{
    parse_cargo_toml, parse_docker_compose, parse_dockerfile, parse_package_json,
    parse_requirements_txt,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct TechnologyProfile {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub databases: Vec<String>,
    pub cloud: Vec<String>,
    pub devops: Vec<String>,
    pub testing: Vec<String>,
}

pub fn detect_technologies_from_files(files: &[(&str, &str)]) -> TechnologyProfile {
    let mut profile = TechnologyProfile::default();

    for (filename, content) in files {
        if filename.ends_with("Cargo.toml") {
            profile.languages.push("Rust".to_string());
            let cargo = parse_cargo_toml(content);
            if cargo.dependencies.contains(&"axum".to_string())
                || cargo.dependencies.contains(&"actix-web".to_string())
            {
                profile.frameworks.push("Axum/Actix".to_string());
            }
            if cargo.dependencies.contains(&"sqlx".to_string()) {
                profile.databases.push("PostgreSQL".to_string());
            }
        } else if filename.ends_with("package.json") {
            let pkg = parse_package_json(content);
            profile.languages.push("JavaScript".to_string());
            if pkg.dev_dependencies.contains(&"typescript".to_string())
                || pkg.dependencies.contains(&"typescript".to_string())
            {
                profile.languages.push("TypeScript".to_string());
            }
            if pkg.dependencies.contains(&"next".to_string()) {
                profile.frameworks.push("Next.js".to_string());
            }
            if pkg.dependencies.contains(&"react".to_string()) {
                profile.frameworks.push("React".to_string());
            }
        } else if filename.ends_with("requirements.txt") {
            profile.languages.push("Python".to_string());
            let reqs = parse_requirements_txt(content);
            if reqs.packages.contains(&"fastapi".to_string()) {
                profile.frameworks.push("FastAPI".to_string());
            }
            if reqs.packages.contains(&"django".to_string()) {
                profile.frameworks.push("Django".to_string());
            }
        } else if filename.ends_with("Dockerfile") {
            let docker = parse_dockerfile(content);
            profile.devops.push("Docker".to_string());
            if docker.is_multistage {
                profile.devops.push("Docker Multi-stage".to_string());
            }
        } else if filename.ends_with("docker-compose.yml")
            || filename.ends_with("docker-compose.yaml")
        {
            let compose = parse_docker_compose(content);
            profile.devops.push("Docker Compose".to_string());
            for db in compose.detected_databases {
                if !profile.databases.contains(&db) {
                    profile.databases.push(db);
                }
            }
        }
    }

    profile.languages.dedup();
    profile.frameworks.dedup();
    profile.databases.dedup();
    profile.devops.dedup();

    profile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector() {
        let files = vec![
            (
                "Cargo.toml",
                "[package]\nname=\"api\"\n[dependencies]\naxum=\"0.7\"\nsqlx=\"0.7\"\n",
            ),
            (
                "docker-compose.yml",
                "version: '3.8'\nservices:\n  postgres:\n    image: postgres:16\n",
            ),
        ];

        let profile = detect_technologies_from_files(&files);
        assert!(profile.languages.contains(&"Rust".to_string()));
        assert!(profile.frameworks.contains(&"Axum/Actix".to_string()));
        assert!(profile.databases.contains(&"PostgreSQL".to_string()));
    }
}
