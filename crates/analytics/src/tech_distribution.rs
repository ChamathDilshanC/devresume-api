use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LanguagePercentage {
    pub language: String,
    pub percentage: f32,
    pub bytes_written: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TechnologyDistribution {
    pub languages: Vec<LanguagePercentage>,
    pub top_frameworks: Vec<String>,
    pub primary_stack: String,
}

pub fn analyze_technology_distribution(
    lang_bytes: &HashMap<String, usize>,
) -> TechnologyDistribution {
    let total_bytes: usize = lang_bytes.values().sum();
    let mut languages = Vec::new();

    for (lang, bytes) in lang_bytes {
        let pct = if total_bytes > 0 {
            (*bytes as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };
        languages.push(LanguagePercentage {
            language: lang.clone(),
            percentage: (pct * 10.0).round() / 10.0,
            bytes_written: *bytes,
        });
    }

    languages.sort_by(|a, b| {
        b.percentage
            .partial_cmp(&a.percentage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let primary_stack = languages
        .first()
        .map(|l| l.language.clone())
        .unwrap_or_else(|| "General Engineering".to_string());

    TechnologyDistribution {
        languages,
        top_frameworks: vec![
            "Axum".to_string(),
            "Next.js".to_string(),
            "PostgreSQL".to_string(),
        ],
        primary_stack,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tech_distribution() {
        let mut map = HashMap::new();
        map.insert("Rust".to_string(), 8000);
        map.insert("TypeScript".to_string(), 2000);

        let dist = analyze_technology_distribution(&map);
        assert_eq!(dist.primary_stack, "Rust");
        assert_eq!(dist.languages[0].percentage, 80.0);
    }
}
