use std::collections::HashMap;

pub struct SkillTaxonomy {
    pub synonym_map: HashMap<String, Vec<String>>,
}

impl Default for SkillTaxonomy {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            "postgres".to_string(),
            vec![
                "postgresql".to_string(),
                "sql".to_string(),
                "rdbms".to_string(),
            ],
        );
        map.insert(
            "postgresql".to_string(),
            vec![
                "postgres".to_string(),
                "sql".to_string(),
                "rdbms".to_string(),
            ],
        );
        map.insert(
            "k8s".to_string(),
            vec!["kubernetes".to_string(), "devops".to_string()],
        );
        map.insert(
            "kubernetes".to_string(),
            vec!["k8s".to_string(), "devops".to_string()],
        );
        map.insert(
            "next.js".to_string(),
            vec![
                "nextjs".to_string(),
                "react".to_string(),
                "frontend".to_string(),
            ],
        );
        map.insert(
            "rust".to_string(),
            vec!["systems programming".to_string(), "backend".to_string()],
        );

        Self { synonym_map: map }
    }
}

impl SkillTaxonomy {
    pub fn is_match(&self, skill_a: &str, skill_b: &str) -> bool {
        let a = skill_a.to_lowercase();
        let b = skill_b.to_lowercase();

        if a == b {
            return true;
        }

        if let Some(synonyms) = self.synonym_map.get(&a) {
            if synonyms.iter().any(|s| s.to_lowercase() == b) {
                return true;
            }
        }

        if let Some(synonyms) = self.synonym_map.get(&b) {
            if synonyms.iter().any(|s| s.to_lowercase() == a) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taxonomy_synonym_matching() {
        let tax = SkillTaxonomy::default();
        assert!(tax.is_match("Postgres", "PostgreSQL"));
        assert!(tax.is_match("k8s", "kubernetes"));
        assert!(tax.is_match("Rust", "rust"));
    }
}
