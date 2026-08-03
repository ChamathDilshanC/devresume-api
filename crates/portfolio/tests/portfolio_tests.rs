use portfolio::{
    build_portfolio_from_resume, render_portfolio_html, Deployer, DeploymentTarget, PortfolioTheme,
};
use resume::{generate_resume_schema, ResumeProject, SkillCategory};

#[tokio::test]
async fn test_end_to_end_portfolio_generation_and_deployment() {
    let resume = generate_resume_schema(
        "Chamath Dilshan",
        "dilshancolonne123@gmail.com",
        "Lead Software Architect",
        "Architecting enterprise AI developer systems in Rust.",
        vec![ResumeProject {
            name: "DevResume AI".to_string(),
            description: "Production developer resume & portfolio generator.".to_string(),
            technologies: vec![
                "Rust".to_string(),
                "Axum".to_string(),
                "PostgreSQL".to_string(),
            ],
            highlights: vec!["Built modular monolith with 17 crates.".to_string()],
            repository_url: Some("https://github.com/ChamathDilshanC/DevResume-AI".to_string()),
        }],
        vec![SkillCategory {
            name: "Languages".to_string(),
            items: vec!["Rust".to_string(), "TypeScript".to_string()],
        }],
    );

    let site = build_portfolio_from_resume("chamath-1", resume, PortfolioTheme::Modern, None);
    assert_eq!(site.user_id, "chamath-1");

    let html = render_portfolio_html(&site);
    assert!(html.contains("Chamath Dilshan"));
    assert!(html.contains("DevResume AI"));

    let dep_result = Deployer::deploy(DeploymentTarget::CloudflarePages, &site.user_id, &html)
        .await
        .expect("Deployment failed");

    assert_eq!(dep_result.status, "published");
    assert!(dep_result.live_url.contains("pages.dev"));
}
