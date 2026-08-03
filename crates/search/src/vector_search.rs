pub fn build_vector_cosine_query(table: &str, limit: usize) -> String {
    format!(
        "SELECT id, entity_id, entity_type, content_chunk, 1 - (embedding <=> $1) AS similarity FROM {} WHERE model = $2 ORDER BY embedding <=> $1 LIMIT {}",
        table, limit
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_query_builder() {
        let q = build_vector_cosine_query("embeddings", 10);
        assert!(q.contains("SELECT id"));
        assert!(q.contains("ORDER BY embedding <=> $1"));
    }
}
