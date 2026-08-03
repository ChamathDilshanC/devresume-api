pub mod builder;
pub mod export;
pub mod parser;
pub mod templates;

pub use builder::{build_resume_json, ResumeData, ResumeProjectData};
pub use export::{export_resume, ExportFormat};
pub use parser::parse_resume_text;
pub use templates::{get_template_style, ResumeTemplate};
