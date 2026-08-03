pub fn build_rag_context(retrieved_chunks: &[String], user_query: &str) -> String {
    format!(
        "Context:\n{}\n\nUser Question: {}",
        retrieved_chunks.join("\n"),
        user_query
    )
}
