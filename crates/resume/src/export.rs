use crate::builder::ResumeSchema;
use crate::renderers::{render_html, render_markdown};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Pdf,
    Docx,
    Png,
    Html,
    Markdown,
    Json,
    Zip,
}

pub fn export_resume_schema(schema: &ResumeSchema, format: ExportFormat) -> Vec<u8> {
    match format {
        ExportFormat::Json => serde_json::to_vec_pretty(schema).unwrap_or_default(),
        ExportFormat::Html => render_html(schema).into_bytes(),
        ExportFormat::Markdown => render_markdown(schema).into_bytes(),
        ExportFormat::Pdf => {
            let html = render_html(schema);
            format!(
                "%PDF-1.4 Mock PDF container wrapping HTML length {}",
                html.len()
            )
            .into_bytes()
        }
        ExportFormat::Docx => {
            let md = render_markdown(schema);
            format!("[DOCX Container wrapping Markdown length {}]", md.len()).into_bytes()
        }
        ExportFormat::Png => format!("PNG Image for {}", schema.basic_info.name).into_bytes(),
        ExportFormat::Zip => format!("ZIP Bundle for {}", schema.basic_info.name).into_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{generate_resume_schema, ResumeProject, SkillCategory};

    #[test]
    fn test_export_formats() {
        let schema = generate_resume_schema(
            "Chamath",
            "c@example.com",
            "Dev",
            "Summary",
            vec![ResumeProject {
                name: "P1".to_string(),
                description: "D1".to_string(),
                technologies: vec!["Rust".to_string()],
                highlights: vec!["H1".to_string()],
                repository_url: None,
            }],
            vec![SkillCategory {
                name: "Core".to_string(),
                items: vec!["Rust".to_string()],
            }],
        );

        let json_bytes = export_resume_schema(&schema, ExportFormat::Json);
        let html_bytes = export_resume_schema(&schema, ExportFormat::Html);
        let md_bytes = export_resume_schema(&schema, ExportFormat::Markdown);
        let pdf_bytes = export_resume_schema(&schema, ExportFormat::Pdf);

        assert!(!json_bytes.is_empty());
        assert!(!html_bytes.is_empty());
        assert!(!md_bytes.is_empty());
        assert!(!pdf_bytes.is_empty());
    }
}
