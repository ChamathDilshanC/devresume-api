use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddingModel {
    OpenAiTextEmbedding3Small,
    OpenAiTextEmbedding3Large,
    OllamaNomicEmbedText,
    MockModel,
}

impl EmbeddingModel {
    pub fn dimension(&self) -> usize {
        match self {
            EmbeddingModel::OpenAiTextEmbedding3Small => 1536,
            EmbeddingModel::OpenAiTextEmbedding3Large => 3072,
            EmbeddingModel::OllamaNomicEmbedText => 768,
            EmbeddingModel::MockModel => 1536,
        }
    }

    pub fn model_name(&self) -> &'static str {
        match self {
            EmbeddingModel::OpenAiTextEmbedding3Small => "text-embedding-3-small",
            EmbeddingModel::OpenAiTextEmbedding3Large => "text-embedding-3-large",
            EmbeddingModel::OllamaNomicEmbedText => "nomic-embed-text",
            EmbeddingModel::MockModel => "mock-embedding-v1",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextChunk {
    pub chunk_index: usize,
    pub content: String,
    pub token_count: usize,
}

pub struct ChunkingEngine {
    pub max_chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Default for ChunkingEngine {
    fn default() -> Self {
        Self {
            max_chunk_size: 512,
            chunk_overlap: 64,
        }
    }
}

impl ChunkingEngine {
    pub fn new(max_chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            max_chunk_size,
            chunk_overlap,
        }
    }

    pub fn chunk_text(&self, text: &str) -> Vec<TextChunk> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return Vec::new();
        }

        let mut chunks = Vec::new();
        let mut i = 0;

        while i < words.len() {
            let end = (i + self.max_chunk_size).min(words.len());
            let chunk_words = &words[i..end];
            let content = chunk_words.join(" ");
            let token_count = chunk_words.len();

            chunks.push(TextChunk {
                chunk_index: chunks.len(),
                content,
                token_count,
            });

            if end == words.len() {
                break;
            }

            i += self
                .max_chunk_size
                .saturating_sub(self.chunk_overlap)
                .max(1);
        }

        chunks
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchEmbeddingJob {
    pub job_id: String,
    pub target_entity: String, // "project", "code", "document", "resume"
    pub entity_id: String,
    pub model: EmbeddingModel,
    pub chunks: Vec<TextChunk>,
    pub status: String, // "pending", "completed", "failed"
}

pub fn generate_chunk_embedding_vector(text: &str, model: EmbeddingModel) -> Vec<f32> {
    let dim = model.dimension();
    let mut vec = vec![0.01f32; dim];
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
    fn test_embedding_model_dimensions() {
        assert_eq!(EmbeddingModel::OpenAiTextEmbedding3Small.dimension(), 1536);
        assert_eq!(EmbeddingModel::OpenAiTextEmbedding3Large.dimension(), 3072);
        assert_eq!(EmbeddingModel::OllamaNomicEmbedText.dimension(), 768);
    }

    #[test]
    fn test_chunking_engine() {
        let engine = ChunkingEngine::new(5, 2);
        let text = "one two three four five six seven eight nine ten";
        let chunks = engine.chunk_text(text);

        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].chunk_index, 0);
        assert!(chunks[0].content.contains("one"));
    }

    #[test]
    fn test_cosine_similarity() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&v1, &v2) - 1.0).abs() < 1e-5);
    }
}
