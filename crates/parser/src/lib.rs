pub mod architecture;
pub mod detector;
pub mod manifest;

pub use architecture::{detect_architecture_pattern, ArchitecturePattern};
pub use detector::{detect_technologies_from_files, TechnologyProfile};
pub use manifest::{
    parse_cargo_toml, parse_docker_compose, parse_dockerfile, parse_package_json, parse_readme,
    parse_requirements_txt, CargoTomlAnalysis, DockerComposeAnalysis, DockerfileAnalysis,
    PackageJsonAnalysis, ReadmeAnalysis, RequirementsTxtAnalysis,
};
