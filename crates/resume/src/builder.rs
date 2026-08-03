use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BasicInfo {
    pub name: String,
    pub email: String,
    pub title: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SkillCategory {
    pub name: String, // e.g. "Languages", "Frameworks", "Databases", "DevOps"
    pub items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ResumeProject {
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub highlights: Vec<String>, // STAR bullet points
    pub repository_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct WorkExperience {
    pub company: String,
    pub role: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub highlights: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EducationItem {
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub graduation_year: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CertificationItem {
    pub name: String,
    pub issuer: String,
    pub year: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SocialLink {
    pub platform: String, // e.g. "GitHub", "LinkedIn"
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ResumeSchema {
    pub basic_info: BasicInfo,
    pub summary: String,
    pub skills: Vec<SkillCategory>,
    pub projects: Vec<ResumeProject>,
    pub experience: Vec<WorkExperience>,
    pub education: Vec<EducationItem>,
    pub certifications: Vec<CertificationItem>,
    pub languages: Vec<String>,
    pub links: Vec<SocialLink>,
}

pub fn generate_resume_schema(
    name: &str,
    email: &str,
    title: &str,
    summary: &str,
    projects: Vec<ResumeProject>,
    skills: Vec<SkillCategory>,
) -> ResumeSchema {
    ResumeSchema {
        basic_info: BasicInfo {
            name: name.to_string(),
            email: email.to_string(),
            title: title.to_string(),
            phone: None,
            location: None,
            website: None,
        },
        summary: summary.to_string(),
        skills,
        projects,
        experience: Vec::new(),
        education: Vec::new(),
        certifications: Vec::new(),
        languages: vec!["English".to_string()],
        links: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resume_schema_generation() {
        let resume = generate_resume_schema(
            "Chamath Dilshan",
            "dilshan@example.com",
            "Senior Backend Engineer",
            "Expert Rust developer",
            vec![ResumeProject {
                name: "DevResume-AI".to_string(),
                description: "AI Resume Platform".to_string(),
                technologies: vec!["Rust".to_string(), "Axum".to_string()],
                highlights: vec!["Architected modular monolith".to_string()],
                repository_url: Some("https://github.com/ChamathDilshanC/DevResume-AI".to_string()),
            }],
            vec![SkillCategory {
                name: "Languages".to_string(),
                items: vec!["Rust".to_string(), "TypeScript".to_string()],
            }],
        );

        assert_eq!(resume.basic_info.name, "Chamath Dilshan");
        assert_eq!(resume.projects.len(), 1);
        assert_eq!(resume.skills.len(), 1);
    }
}
