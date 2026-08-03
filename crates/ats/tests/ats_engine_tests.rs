use ats::generate_ats_report;
use resume::{generate_resume_schema, ResumeProject, SkillCategory};

#[test]
fn test_end_to_end_ats_analysis() {
    let resume = generate_resume_schema(
        "Chamath Dilshan",
        "dilshancolonne123@gmail.com",
        "Lead Systems & AI Engineer",
        "Lead engineer specializing in Rust backend microservices, PostgreSQL, and LLM orchestration.",
        vec![ResumeProject {
            name: "DevResume AI".to_string(),
            description: "Production platform for AI-powered developer resumes and portfolios.".to_string(),
            technologies: vec![
                "Rust".to_string(),
                "Axum".to_string(),
                "PostgreSQL".to_string(),
                "Docker".to_string(),
            ],
            highlights: vec![
                "Architected modular monolith with 17 decoupled crates.".to_string(),
                "Built component-based ATS scoring engine with skill taxonomy matching.".to_string(),
            ],
            repository_url: Some("https://github.com/ChamathDilshanC/DevResume-AI".to_string()),
        }],
        vec![
            SkillCategory {
                name: "Languages".to_string(),
                items: vec!["Rust".to_string(), "TypeScript".to_string(), "Python".to_string()],
            },
            SkillCategory {
                name: "Databases".to_string(),
                items: vec!["PostgreSQL".to_string(), "Redis".to_string()],
            },
        ],
    );

    let job_description = r#"
We are hiring a Lead Rust Backend Engineer with 5+ years experience.
Key Requirements:
- Deep expertise in Rust, Axum, and Tokio.
- Experience with PostgreSQL, Redis, and pgvector.
- Hands-on experience with Docker and Kubernetes for container orchestration.
"#;

    let report = generate_ats_report(&resume, job_description, "v1.0.0");

    assert!(report.overall_score >= 70);
    assert_eq!(report.breakdown.structure_score, 5);
    assert_eq!(report.breakdown.formatting_score, 5);
    assert!(report.matched_skills.contains(&"rust".to_string()));
    assert!(report.matched_skills.contains(&"postgresql".to_string()));
    assert!(report.missing_skills.contains(&"kubernetes".to_string()));
    assert!(!report.recommendations.is_empty());
}
