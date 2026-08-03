use crate::parser::{parse_job_description, NormalizedJobProfile};
use crate::scorer::{ComponentScorer, ScoreBreakdown};
use md5::Digest;
use resume::ResumeSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtsReport {
    pub report_id: String,
    pub resume_version: String,
    pub job_description_hash: String,
    pub overall_score: u32,
    pub breakdown: ScoreBreakdown,
    pub matched_skills: Vec<String>,
    pub missing_skills: Vec<String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub recommendations: Vec<String>,
    pub generated_at: String,
}

pub fn generate_ats_report(
    resume: &ResumeSchema,
    job_description_text: &str,
    resume_version: &str,
) -> AtsReport {
    let job_profile: NormalizedJobProfile = parse_job_description(job_description_text);
    let scorer = ComponentScorer::default();

    let (breakdown, matched_skills, missing_skills) = scorer.evaluate(resume, &job_profile);

    let mut strengths = Vec::new();
    let mut weaknesses = Vec::new();
    let mut recommendations = Vec::new();

    if !matched_skills.is_empty() {
        strengths.push(format!(
            "Strong match on core skills: {}",
            matched_skills.join(", ")
        ));
    }

    if !missing_skills.is_empty() {
        weaknesses.push(format!("Missing keywords: {}", missing_skills.join(", ")));
        for missing in &missing_skills {
            recommendations.push(format!(
                "Consider adding experience or projects featuring '{}'",
                missing
            ));
        }
    }

    let jd_hash = format!("{:x}", md5::Md5::digest(job_description_text.as_bytes()));

    AtsReport {
        report_id: format!("ats-{}", uuid::Uuid::new_v4()),
        resume_version: resume_version.to_string(),
        job_description_hash: jd_hash,
        overall_score: breakdown.total_score,
        breakdown,
        matched_skills,
        missing_skills,
        strengths,
        weaknesses,
        recommendations,
        generated_at: chrono::Utc::now().to_rfc3339(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resume::{generate_resume_schema, ResumeProject, SkillCategory};

    #[test]
    fn test_ats_report_generation() {
        let resume = generate_resume_schema(
            "Chamath",
            "c@example.com",
            "Software Engineer",
            "Rust developer",
            vec![ResumeProject {
                name: "P1".to_string(),
                description: "D1".to_string(),
                technologies: vec!["Rust".to_string()],
                highlights: vec!["Built system".to_string()],
                repository_url: None,
            }],
            vec![SkillCategory {
                name: "Backend".to_string(),
                items: vec!["Rust".to_string(), "PostgreSQL".to_string()],
            }],
        );

        let jd = "Looking for a Rust engineer with Docker and PostgreSQL knowledge.";
        let report = generate_ats_report(&resume, jd, "v1.0");

        assert!(report.overall_score > 70);
        assert!(!report.report_id.is_empty());
        assert_eq!(report.resume_version, "v1.0");
    }
}
