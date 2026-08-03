use crate::builder::ResumeSchema;

pub fn render_html(schema: &ResumeSchema) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n");
    html.push_str(&format!(
        "<title>{} - Resume</title>\n",
        schema.basic_info.name
    ));
    html.push_str("<style>body{font-family:sans-serif;margin:40px;color:#111;} h1{color:#0066cc;} .section{margin-top:24px;}</style>\n");
    html.push_str("</head>\n<body>\n");

    // Basic Info
    html.push_str(&format!("<h1>{}</h1>\n", schema.basic_info.name));
    html.push_str(&format!(
        "<h3>{} | {}</h3>\n",
        schema.basic_info.title, schema.basic_info.email
    ));

    // Summary
    html.push_str("<div class=\"section\">\n<h2>Summary</h2>\n");
    html.push_str(&format!("<p>{}</p>\n</div>\n", schema.summary));

    // Skills
    if !schema.skills.is_empty() {
        html.push_str("<div class=\"section\">\n<h2>Skills</h2>\n<ul>\n");
        for cat in &schema.skills {
            html.push_str(&format!(
                "<li><strong>{}:</strong> {}</li>\n",
                cat.name,
                cat.items.join(", ")
            ));
        }
        html.push_str("</ul>\n</div>\n");
    }

    // Projects
    if !schema.projects.is_empty() {
        html.push_str("<div class=\"section\">\n<h2>Projects</h2>\n");
        for proj in &schema.projects {
            html.push_str(&format!("<h3>{}</h3>\n", proj.name));
            html.push_str(&format!("<p>{}</p>\n", proj.description));
            html.push_str(&format!(
                "<p><em>Technologies:</em> {}</p>\n",
                proj.technologies.join(", ")
            ));
            if !proj.highlights.is_empty() {
                html.push_str("<ul>\n");
                for hl in &proj.highlights {
                    html.push_str(&format!("<li>{}</li>\n", hl));
                }
                html.push_str("</ul>\n");
            }
        }
        html.push_str("</div>\n");
    }

    html.push_str("</body>\n</html>");
    html
}

pub fn render_markdown(schema: &ResumeSchema) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", schema.basic_info.name));
    md.push_str(&format!(
        "**{}** | {}\n\n",
        schema.basic_info.title, schema.basic_info.email
    ));
    md.push_str(&format!("## Summary\n{}\n\n", schema.summary));

    if !schema.skills.is_empty() {
        md.push_str("## Skills\n");
        for cat in &schema.skills {
            md.push_str(&format!("- **{}:** {}\n", cat.name, cat.items.join(", ")));
        }
        md.push('\n');
    }

    if !schema.projects.is_empty() {
        md.push_str("## Projects\n");
        for proj in &schema.projects {
            md.push_str(&format!("### {}\n{}\n", proj.name, proj.description));
            md.push_str(&format!(
                "*Technologies:* {}\n",
                proj.technologies.join(", ")
            ));
            for hl in &proj.highlights {
                md.push_str(&format!("- {}\n", hl));
            }
            md.push('\n');
        }
    }

    md
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{generate_resume_schema, ResumeProject, SkillCategory};

    #[test]
    fn test_render_html_and_markdown() {
        let resume = generate_resume_schema(
            "Chamath Dilshan",
            "dilshan@example.com",
            "Software Engineer",
            "Summary text",
            vec![ResumeProject {
                name: "DevResume".to_string(),
                description: "AI Platform".to_string(),
                technologies: vec!["Rust".to_string()],
                highlights: vec!["Highlight 1".to_string()],
                repository_url: None,
            }],
            vec![SkillCategory {
                name: "Backend".to_string(),
                items: vec!["Rust".to_string()],
            }],
        );

        let html = render_html(&resume);
        let md = render_markdown(&resume);

        assert!(html.contains("<h1>Chamath Dilshan</h1>"));
        assert!(md.contains("# Chamath Dilshan"));
    }
}
