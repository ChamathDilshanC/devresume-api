use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct NormalizedJobProfile {
    pub title: String,
    pub required_skills: Vec<String>,
    pub min_years_experience: usize,
    pub tools_and_frameworks: Vec<String>,
    pub soft_skills: Vec<String>,
}

pub fn parse_job_description(jd_text: &str) -> NormalizedJobProfile {
    let lower = jd_text.to_lowercase();
    let mut profile = NormalizedJobProfile::default();

    // Skill extraction heuristics
    let tech_keywords = vec![
        "rust",
        "postgresql",
        "postgres",
        "docker",
        "kubernetes",
        "k8s",
        "next.js",
        "react",
        "typescript",
        "javascript",
        "python",
        "fastapi",
        "django",
        "aws",
        "redis",
        "mongodb",
        "sqlx",
        "axum",
        "git",
        "ci/cd",
        "terraform",
    ];

    for kw in tech_keywords {
        if lower.contains(kw) {
            profile.required_skills.push(kw.to_string());
        }
    }

    // Experience extraction heuristic
    if lower.contains("5+ years") || lower.contains("5 years") {
        profile.min_years_experience = 5;
    } else if lower.contains("3+ years") || lower.contains("3 years") {
        profile.min_years_experience = 3;
    } else if lower.contains("1+ years") || lower.contains("1 year") {
        profile.min_years_experience = 1;
    }

    profile.required_skills.dedup();
    profile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_description_parser() {
        let jd = "Looking for a Senior Rust Backend Engineer with 5+ years experience using PostgreSQL, Docker, and Kubernetes.";
        let profile = parse_job_description(jd);

        assert_eq!(profile.min_years_experience, 5);
        assert!(profile.required_skills.contains(&"rust".to_string()));
        assert!(profile.required_skills.contains(&"postgresql".to_string()));
        assert!(profile.required_skills.contains(&"docker".to_string()));
    }
}
