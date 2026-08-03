pub mod agents;
pub mod embeddings;
pub mod prompt_engine;
pub mod provider;
pub mod rag;
pub mod tokenizer;

pub use embeddings::{
    cosine_similarity, generate_chunk_embedding_vector, BatchEmbeddingJob, ChunkingEngine,
    EmbeddingModel, TextChunk,
};
pub use prompt_engine::{
    build_ats_analysis_prompt, build_project_summary_prompt, build_resume_prompt, ProjectContext,
};
pub use provider::{
    create_ai_provider, AIProvider, ChatMessage, ClaudeProvider, GeminiProvider, MockAIProvider,
    OllamaProvider, OpenAIProvider,
};
