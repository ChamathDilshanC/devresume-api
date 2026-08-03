pub fn build_fulltext_search_query(query: &str) -> String {
    format!(
        "SELECT id, title FROM projects WHERE to_tsvector('english', name || ' ' || COALESCE(description, '')) @@ plainto_tsquery('english', '{}')",
        query
    )
}
