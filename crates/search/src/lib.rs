use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub repository_id: String,
    pub title: String,
    pub similarity_score: f32,
}

pub fn format_vector_query(query: &str) -> String {
    format!("SELECT repository_id, content_chunk FROM repository_embeddings WHERE content_chunk LIKE '%{}%'", query)
}
