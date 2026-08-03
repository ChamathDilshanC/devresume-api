use crate::seo::SeoMetadata;
use crate::theme::PortfolioTheme;
use resume::ResumeSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioSite {
    pub user_id: String,
    pub title: String,
    pub headline: String,
    pub theme: PortfolioTheme,
    pub resume_schema: ResumeSchema,
    pub seo: SeoMetadata,
}

pub fn build_portfolio_from_resume(
    user_id: &str,
    resume: ResumeSchema,
    theme: PortfolioTheme,
    custom_domain: Option<&str>,
) -> PortfolioSite {
    let domain = custom_domain
        .map(|d| d.to_string())
        .unwrap_or_else(|| format!("https://portfolio.devresume.ai/{}", user_id));

    let seo = SeoMetadata {
        title: format!("{} | {}", resume.basic_info.name, resume.basic_info.title),
        description: resume.summary.clone(),
        canonical_url: domain,
        og_image_url: None,
        twitter_handle: None,
    };

    PortfolioSite {
        user_id: user_id.to_string(),
        title: format!("{} - Engineering Portfolio", resume.basic_info.name),
        headline: resume.basic_info.title.clone(),
        theme,
        resume_schema: resume,
        seo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resume::{generate_resume_schema, ResumeProject, SkillCategory};

    #[test]
    fn test_build_portfolio_from_resume_schema() {
        let resume = generate_resume_schema(
            "Chamath Dilshan",
            "dilshan@example.com",
            "Senior Backend Engineer",
            "Rust AI backend engineer",
            vec![ResumeProject {
                name: "DevResume-AI".to_string(),
                description: "AI Platform".to_string(),
                technologies: vec!["Rust".to_string()],
                highlights: vec!["Built modular monolith".to_string()],
                repository_url: None,
            }],
            vec![SkillCategory {
                name: "Languages".to_string(),
                items: vec!["Rust".to_string()],
            }],
        );

        let portfolio =
            build_portfolio_from_resume("user-100", resume, PortfolioTheme::Modern, None);
        assert_eq!(portfolio.user_id, "user-100");
        assert!(portfolio.title.contains("Chamath Dilshan"));
    }
}
