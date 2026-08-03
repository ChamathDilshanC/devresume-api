use parser::{
    detect_architecture_pattern, detect_technologies_from_files, parse_cargo_toml, parse_readme,
    ArchitecturePattern,
};

#[test]
fn test_end_to_end_repository_parsing() {
    let files = vec![
        (
            "Cargo.toml",
            "[package]\nname=\"devresume-api\"\n[dependencies]\naxum=\"0.7\"\nsqlx=\"0.7\"\n",
        ),
        (
            "package.json",
            "{\"name\":\"web\",\"dependencies\":{\"next\":\"14.0.0\"},\"devDependencies\":{\"typescript\":\"5.0.0\"}}",
        ),
        (
            "Dockerfile",
            "FROM rust:1.78 as builder\nEXPOSE 8080\nFROM debian:bookworm-slim\n",
        ),
        (
            "docker-compose.yml",
            "version: '3'\nservices:\n  postgres:\n    image: postgres:16\n  redis:\n    image: redis:7\n",
        ),
        (
            "README.md",
            "# DevResume AI\nAI-powered developer profile platform.\n",
        ),
    ];

    let profile = detect_technologies_from_files(&files);
    assert!(profile.languages.contains(&"Rust".to_string()));
    assert!(profile.languages.contains(&"JavaScript".to_string()));
    assert!(profile.languages.contains(&"TypeScript".to_string()));
    assert!(profile.frameworks.contains(&"Axum/Actix".to_string()));
    assert!(profile.frameworks.contains(&"Next.js".to_string()));
    assert!(profile.databases.contains(&"PostgreSQL".to_string()));
    assert!(profile.databases.contains(&"Redis".to_string()));
    assert!(profile.devops.contains(&"Docker".to_string()));

    let filenames: Vec<&str> = files.iter().map(|(name, _)| *name).collect();
    let pattern = detect_architecture_pattern(&filenames);
    assert_eq!(pattern, ArchitecturePattern::Monolith);

    let cargo = parse_cargo_toml(files[0].1);
    assert_eq!(cargo.package_name, Some("devresume-api".to_string()));

    let readme = parse_readme(files[4].1);
    assert_eq!(readme.title, Some("DevResume AI".to_string()));
}
