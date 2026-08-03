pub fn generate_chunk_embedding_vector(text: &str) -> Vec<f32> {
    let mut vec = vec![0.01f32; 1536];
    if !text.is_empty() {
        let hash = text.bytes().map(|b| b as usize).sum::<usize>();
        vec[0] = (hash % 100) as f32 / 100.0;
        vec[1] = text.len() as f32 / 1000.0;
    }
    vec
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical_vectors() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&v1, &v2);
        assert!((sim - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_similarity_orthogonal_vectors() {
        let v1 = vec![1.0, 0.0];
        let v2 = vec![0.0, 1.0];
        let sim = cosine_similarity(&v1, &v2);
        assert!((sim - 0.0).abs() < 1e-5);
    }
}
