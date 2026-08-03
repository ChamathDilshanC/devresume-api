pub fn build_vector_cosine_query(table: &str, limit: usize) -> String {
    format!(
        "SELECT id, content_chunk, 1 - (embedding <=> $1) AS similarity FROM {} ORDER BY embedding <=> $1 LIMIT {}",
        table, limit
    )
}
