use crate::builder::PortfolioSite;

pub fn render_portfolio_html(site: &PortfolioSite) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");

    // SEO tags
    html.push_str(&site.seo.generate_meta_tags());
    html.push_str(&site.seo.generate_json_ld(
        &site.resume_schema.basic_info.name,
        &site.resume_schema.basic_info.title,
    ));

    // CSS Theme variables
    html.push_str(&format!("<!-- Theme: {} -->\n", site.theme.name()));
    html.push_str("<style>\n");
    html.push_str(":root {\n");
    html.push_str(site.theme.css_variables());
    html.push_str("\n}\n");

    html.push_str("body { background-color: var(--bg); color: var(--text); font-family: system-ui, sans-serif; margin: 0; padding: 0; line-height: 1.6; }\n");
    html.push_str(".container { max-width: 900px; margin: 40px auto; padding: 20px; }\n");
    html.push_str("header { border-bottom: 2px solid var(--accent); padding-bottom: 20px; margin-bottom: 40px; }\n");
    html.push_str("h1 { color: var(--accent); font-size: 2.5rem; margin: 0 0 10px 0; }\n");
    html.push_str(".project-card { background: var(--card); border-radius: 8px; padding: 20px; margin-bottom: 20px; border: 1px solid rgba(255,255,255,0.1); }\n");
    html.push_str(".badge { background: var(--accent); color: var(--bg); padding: 4px 8px; border-radius: 4px; font-size: 0.8rem; margin-right: 6px; font-weight: bold; }\n");
    html.push_str("</style>\n</head>\n<body>\n");

    html.push_str("<div class=\"container\">\n");
    html.push_str("<header>\n");
    html.push_str(&format!(
        "<h1>{}</h1>\n",
        site.resume_schema.basic_info.name
    ));
    html.push_str(&format!("<h2>{}</h2>\n", site.headline));
    html.push_str(&format!("<p>{}</p>\n", site.resume_schema.summary));
    html.push_str("</header>\n");

    // Featured Projects
    if !site.resume_schema.projects.is_empty() {
        html.push_str("<section>\n<h2>Featured Projects</h2>\n");
        for proj in &site.resume_schema.projects {
            html.push_str("<div class=\"project-card\">\n");
            html.push_str(&format!("<h3>{}</h3>\n", proj.name));
            html.push_str(&format!("<p>{}</p>\n", proj.description));
            html.push_str("<div>\n");
            for tech in &proj.technologies {
                html.push_str(&format!("<span class=\"badge\">{}</span>", tech));
            }
            html.push_str("</div>\n</div>\n");
        }
        html.push_str("</section>\n");
    }

    html.push_str("</div>\n</body>\n</html>");
    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::build_portfolio_from_resume;
    use crate::theme::PortfolioTheme;
    use resume::{generate_resume_schema, ResumeProject, SkillCategory};

    #[test]
    fn test_render_portfolio_html() {
        let resume = generate_resume_schema(
            "Chamath Dilshan",
            "dilshan@example.com",
            "Full Stack Engineer",
            "Portfolio builder test",
            vec![ResumeProject {
                name: "DevResume".to_string(),
                description: "Description".to_string(),
                technologies: vec!["Rust".to_string()],
                highlights: vec![],
                repository_url: None,
            }],
            vec![SkillCategory {
                name: "Tech".to_string(),
                items: vec!["Rust".to_string()],
            }],
        );

        let site = build_portfolio_from_resume("user-1", resume, PortfolioTheme::Glass, None);
        let html = render_portfolio_html(&site);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Chamath Dilshan"));
        assert!(html.contains("glassmorphism"));
    }
}
