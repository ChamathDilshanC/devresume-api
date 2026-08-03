use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DetectedTechnologies {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub databases: Vec<String>,
    pub devops: Vec<String>,
}

pub fn detect_technologies_from_filenames(filenames: &[&str]) -> DetectedTechnologies {
    let mut detected = DetectedTechnologies::default();

    for file in filenames {
        if file.contains("Cargo.toml") {
            detected.languages.push("Rust".to_string());
        }
        if file.contains("package.json") {
            detected.languages.push("JavaScript/TypeScript".to_string());
        }
        if file.contains("Dockerfile") || file.contains("docker-compose") {
            detected.devops.push("Docker".to_string());
        }
        if file.contains("requirements.txt") || file.contains("Pyproject.toml") {
            detected.languages.push("Python".to_string());
        }
    }

    detected
}
