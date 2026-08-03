pub fn build_fulltext_search_query(table: &str, limit: usize) -> String {
    format!(
        "SELECT id, title, content, ts_rank(to_tsvector('english', title || ' ' || COALESCE(content, '')), plainto_tsquery('english', $1)) AS rank FROM {} WHERE to_tsvector('english', title || ' ' || COALESCE(content, '')) @@ plainto_tsquery('english', $1) ORDER BY rank DESC LIMIT {}",
        table, limit
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fulltext_query_builder() {
        let q = build_fulltext_search_query("projects", 20);
        assert!(q.contains("to_tsvector"));
        assert!(q.contains("plainto_tsquery"));
    }
}
