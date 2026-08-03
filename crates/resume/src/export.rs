use crate::builder::ResumeData;

pub enum ExportFormat {
    Pdf,
    Docx,
    Png,
    Html,
    Markdown,
    Zip,
}

pub fn export_resume(data: &ResumeData, format: ExportFormat) -> Vec<u8> {
    match format {
        ExportFormat::Pdf => format!("PDF Export for {}", data.name).into_bytes(),
        ExportFormat::Docx => format!("DOCX Export for {}", data.name).into_bytes(),
        ExportFormat::Png => format!("PNG Image for {}", data.name).into_bytes(),
        ExportFormat::Html => format!("<html><body><h1>{}</h1></body></html>", data.name).into_bytes(),
        ExportFormat::Markdown => format!("# {}\n{}", data.name, data.summary).into_bytes(),
        ExportFormat::Zip => format!("ZIP Bundle for {}", data.name).into_bytes(),
    }
}
