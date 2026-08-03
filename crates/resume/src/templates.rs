pub enum ResumeTemplate {
    Modern,
    Executive,
    AtsMinimal,
}

pub fn get_template_style(template: ResumeTemplate) -> &'static str {
    match template {
        ResumeTemplate::Modern => "modern-dark-slate",
        ResumeTemplate::Executive => "executive-serif-classic",
        ResumeTemplate::AtsMinimal => "ats-plain-monospaced",
    }
}
