use resume::{
    export_resume_schema, generate_resume_schema, render_html, render_markdown, ExportFormat,
    ResumeProject, SkillCategory,
};

#[test]
fn test_end_to_end_resume_builder_and_exports() {
    let schema = generate_resume_schema(
        "Chamath Dilshan",
        "dilshancolonne123@gmail.com",
        "Lead Backend Engineer",
        "Production-ready Rust backend engineer focused on AI platforms.",
        vec![ResumeProject {
            name: "DevResume AI".to_string(),
            description: "Developer resume and portfolio generation platform.".to_string(),
            technologies: vec![
                "Rust".to_string(),
                "Axum".to_string(),
                "PostgreSQL".to_string(),
                "Next.js".to_string(),
            ],
            highlights: vec![
                "Architected modular monolith in Rust with 17 decoupled crates.".to_string(),
                "Implemented hybrid RRF search engine combining full text search and pgvector cosine similarity.".to_string(),
            ],
            repository_url: Some("https://github.com/ChamathDilshanC/DevResume-AI".to_string()),
        }],
        vec![
            SkillCategory {
                name: "Languages".to_string(),
                items: vec!["Rust".to_string(), "TypeScript".to_string(), "Python".to_string()],
            },
            SkillCategory {
                name: "Frameworks".to_string(),
                items: vec!["Axum".to_string(), "Next.js".to_string(), "FastAPI".to_string()],
            },
        ],
    );

    let html = render_html(&schema);
    let md = render_markdown(&schema);

    assert!(html.contains("Chamath Dilshan"));
    assert!(html.contains("DevResume AI"));
    assert!(md.contains("# Chamath Dilshan"));
    assert!(md.contains("Lead Backend Engineer"));

    let json_bytes = export_resume_schema(&schema, ExportFormat::Json);
    let html_bytes = export_resume_schema(&schema, ExportFormat::Html);
    let md_bytes = export_resume_schema(&schema, ExportFormat::Markdown);
    let pdf_bytes = export_resume_schema(&schema, ExportFormat::Pdf);
    let docx_bytes = export_resume_schema(&schema, ExportFormat::Docx);

    assert!(!json_bytes.is_empty());
    assert!(!html_bytes.is_empty());
    assert!(!md_bytes.is_empty());
    assert!(!pdf_bytes.is_empty());
    assert!(!docx_bytes.is_empty());
}
