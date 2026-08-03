use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SeoMetadata {
    pub title: String,
    pub description: String,
    pub canonical_url: String,
    pub og_image_url: Option<String>,
    pub twitter_handle: Option<String>,
}

impl SeoMetadata {
    pub fn generate_meta_tags(&self) -> String {
        let mut tags = String::new();
        tags.push_str(&format!("<title>{}</title>\n", self.title));
        tags.push_str(&format!(
            "<meta name=\"description\" content=\"{}\">\n",
            self.description
        ));
        tags.push_str(&format!(
            "<link rel=\"canonical\" href=\"{}\">\n",
            self.canonical_url
        ));

        // OpenGraph
        tags.push_str(&format!(
            "<meta property=\"og:title\" content=\"{}\">\n",
            self.title
        ));
        tags.push_str(&format!(
            "<meta property=\"og:description\" content=\"{}\">\n",
            self.description
        ));
        tags.push_str(&format!(
            "<meta property=\"og:url\" content=\"{}\">\n",
            self.canonical_url
        ));
        if let Some(ref img) = self.og_image_url {
            tags.push_str(&format!(
                "<meta property=\"og:image\" content=\"{}\">\n",
                img
            ));
        }

        // Twitter Cards
        tags.push_str("<meta name=\"twitter:card\" content=\"summary_large_image\">\n");
        tags.push_str(&format!(
            "<meta name=\"twitter:title\" content=\"{}\">\n",
            self.title
        ));
        if let Some(ref handle) = self.twitter_handle {
            tags.push_str(&format!(
                "<meta name=\"twitter:site\" content=\"{}\">\n",
                handle
            ));
        }

        tags
    }

    pub fn generate_json_ld(&self, name: &str, title: &str) -> String {
        format!(
            r#"<script type="application/ld+json">
{{
  "@context": "https://schema.org",
  "@type": "Person",
  "name": "{}",
  "jobTitle": "{}",
  "url": "{}"
}}
</script>"#,
            name, title, self.canonical_url
        )
    }

    pub fn generate_robots_txt() -> &'static str {
        "User-agent: *\nAllow: /\nSitemap: /sitemap.xml\n"
    }

    pub fn generate_sitemap(url: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>{}</loc>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>"#,
            url
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seo_metadata_generation() {
        let seo = SeoMetadata {
            title: "Chamath Dilshan | Software Engineer".to_string(),
            description: "Developer Portfolio".to_string(),
            canonical_url: "https://chamath.dev".to_string(),
            og_image_url: Some("https://chamath.dev/og.png".to_string()),
            twitter_handle: Some("@chamath".to_string()),
        };

        let tags = seo.generate_meta_tags();
        assert!(tags.contains("og:title"));
        assert!(tags.contains("canonical"));

        let json_ld = seo.generate_json_ld("Chamath", "Engineer");
        assert!(json_ld.contains("https://schema.org"));
    }
}
