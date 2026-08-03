pub fn generate_chunk_embedding_vector(text: &str) -> Vec<f32> {
    let mut vec = vec![0.0f32; 1536];
    if !text.is_empty() {
        vec[0] = text.len() as f32 / 100.0;
    }
    vec
}
