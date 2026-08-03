# ADR 0005 — AI Provider Abstraction via Trait

- **Status**: Accepted
- **Date**: 2026-01-01
- **Author**: ChamathDilshanC

---

## Context

DevResume AI uses large language models for resume generation, career recommendations, interview practice, ATS analysis, and embedding generation. AI providers (OpenAI, Gemini, Claude, Ollama) have different APIs, pricing, rate limits, and capabilities. Hardcoding a single provider creates vendor lock-in and makes it impossible to swap providers as the market evolves.

---

## Decision

**Define an `AIProvider` trait in `crates/ai/` that all provider implementations must satisfy.**

---

## Trait Definition

```rust
#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate(&self, req: GenerateRequest) -> Result<GenerateResponse, AIError>;
    async fn embeddings(&self, req: EmbeddingRequest) -> Result<EmbeddingResponse, AIError>;
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, AIError>;
    async fn stream(&self, req: GenerateRequest) -> Result<BoxStream<'_, StreamChunk>, AIError>;
    fn model_name(&self) -> &str;
    fn provider_name(&self) -> &str;
    fn max_tokens(&self) -> u32;
}
```

---

## Provider Registry

```rust
pub enum ProviderKind {
    OpenAI,
    Gemini,
    Claude,
    Ollama,
}

// Selected at runtime from config
pub fn build_provider(config: &AIConfig) -> Arc<dyn AIProvider> {
    match config.provider {
        ProviderKind::OpenAI => Arc::new(OpenAIProvider::new(config)),
        ProviderKind::Gemini => Arc::new(GeminiProvider::new(config)),
        ProviderKind::Claude => Arc::new(ClaudeProvider::new(config)),
        ProviderKind::Ollama => Arc::new(OllamaProvider::new(config)),
    }
}
```

---

## Rationale

| Criterion | Trait Abstraction | Hardcoded Provider |
|-----------|------------------|--------------------|
| Vendor lock-in | ✅ None | ❌ High |
| Test isolation | ✅ Mock provider | ❌ Requires real API |
| Provider swap | ✅ Config change | ❌ Code change |
| A/B testing providers | ✅ Possible | ❌ Not possible |
| Cost optimization | ✅ Route by cost | ❌ Fixed |

---

## Consequences

- All AI calls must go through `Arc<dyn AIProvider>` — never call OpenAI SDK directly from domain crates.
- `crates/ai` must have a `MockAIProvider` for testing that returns deterministic responses.
- Provider selection is controlled by `AI__PROVIDER` environment variable.
- Token usage must be tracked per request via `GenerateResponse.usage`.
- **Do not add direct OpenAI/Gemini/Claude SDK calls outside `crates/ai/`.**
