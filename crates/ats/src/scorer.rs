use crate::parser::NormalizedJobProfile;
use crate::taxonomy::SkillTaxonomy;
use resume::ResumeSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ScoreBreakdown {
    pub required_skills_score: u32, // Max 30
    pub experience_score: u32,      // Max 20
    pub keywords_score: u32,        // Max 15
    pub education_score: u32,       // Max 10
    pub projects_score: u32,        // Max 10
    pub structure_score: u32,       // Max 5
    pub formatting_score: u32,      // Max 5
    pub readability_score: u32,     // Max 5
    pub total_score: u32,           // Max 100
}

#[derive(Default)]
pub struct ComponentScorer {
    pub taxonomy: SkillTaxonomy,
}

impl ComponentScorer {
    pub fn evaluate(
        &self,
        resume: &ResumeSchema,
        job: &NormalizedJobProfile,
    ) -> (ScoreBreakdown, Vec<String>, Vec<String>) {
        let mut matched_skills = Vec::new();
        let mut missing_skills = Vec::new();

        let resume_skills: Vec<String> = resume
            .skills
            .iter()
            .flat_map(|cat| cat.items.clone())
            .collect();

        // 1. Required Skills Match (Max 30)
        let mut skill_points = 0u32;
        if !job.required_skills.is_empty() {
            for req in &job.required_skills {
                if resume_skills.iter().any(|s| self.taxonomy.is_match(s, req)) {
                    matched_skills.push(req.clone());
                    skill_points += 1;
                } else {
                    missing_skills.push(req.clone());
                }
            }

            skill_points =
                ((skill_points as f32 / job.required_skills.len() as f32) * 30.0).round() as u32;
        } else {
            skill_points = 30; // Default full points if no explicit reqs
        }

        // 2. Experience Score (Max 20)
        let experience_score = if resume.projects.len() >= 2 || !resume.experience.is_empty() {
            20
        } else {
            10
        };

        // 3. Keywords Score (Max 15)
        let keywords_score = if !matched_skills.is_empty() { 15 } else { 8 };

        // 4. Education Score (Max 10)
        let education_score = 10;

        // 5. Projects Score (Max 10)
        let projects_score = if !resume.projects.is_empty() { 10 } else { 5 };

        // 6. Structure, Formatting, Readability (5 each)
        let structure_score = 5;
        let formatting_score = 5;
        let readability_score = 5;

        let total_score = (skill_points
            + experience_score
            + keywords_score
            + education_score
            + projects_score
            + structure_score
            + formatting_score
            + readability_score)
            .min(100);

        let breakdown = ScoreBreakdown {
            required_skills_score: skill_points,
            experience_score,
            keywords_score,
            education_score,
            projects_score,
            structure_score,
            formatting_score,
            readability_score,
            total_score,
        };

        (breakdown, matched_skills, missing_skills)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resume::{generate_resume_schema, ResumeProject, SkillCategory};

    #[test]
    fn test_component_scoring() {
        let scorer = ComponentScorer::default();
        let resume = generate_resume_schema(
            "Chamath",
            "c@example.com",
            "Rust Dev",
            "Summary",
            vec![ResumeProject {
                name: "P1".to_string(),
                description: "D1".to_string(),
                technologies: vec!["Rust".to_string()],
                highlights: vec!["Built backend".to_string()],
                repository_url: None,
            }],
            vec![SkillCategory {
                name: "Core".to_string(),
                items: vec!["Rust".to_string(), "PostgreSQL".to_string()],
            }],
        );

        let job = NormalizedJobProfile {
            title: "Rust Developer".to_string(),
            required_skills: vec![
                "rust".to_string(),
                "postgresql".to_string(),
                "docker".to_string(),
            ],
            min_years_experience: 3,
            tools_and_frameworks: vec![],
            soft_skills: vec![],
        };

        let (breakdown, matched, missing) = scorer.evaluate(&resume, &job);
        assert!(breakdown.total_score >= 70);
        assert!(matched.contains(&"rust".to_string()));
        assert!(missing.contains(&"docker".to_string()));
    }
}
