# ADR 0003 — Use pgvector for Semantic Embeddings

- **Status**: Accepted
- **Date**: 2026-01-01
- **Author**: ChamathDilshanC

---

## Context

DevResume AI requires semantic search across repositories, projects, skills, resumes, and code. This demands storing and querying high-dimensional embedding vectors (1536-dim for OpenAI text-embedding-3-small, 768-dim for others).

Candidates evaluated:
- **pgvector** — PostgreSQL extension for vector storage
- **Qdrant** — dedicated vector database
- **Weaviate** — GraphQL vector DB
- **Milvus** — distributed vector DB
- **Pinecone** — managed vector DB (SaaS)

---

## Decision

**Use pgvector as a PostgreSQL extension for all embedding storage and similarity search.**

---

## Rationale

| Criterion | pgvector | Qdrant | Pinecone |
|-----------|---------|--------|---------|
| Infrastructure complexity | ✅ Zero (reuses PG) | ⚠️ New service | ⚠️ Managed SaaS |
| SQL join capability | ✅ Native JOIN | ❌ No | ❌ No |
| ACID transactions | ✅ Full | ❌ No | ❌ No |
| Hybrid search (FTS + vector) | ✅ Same query | ❌ Separate systems | ❌ Separate systems |
| Self-hosted | ✅ Yes | ✅ Yes | ❌ Cloud-only |
| Operational cost | ✅ Minimal | ⚠️ Additional service | 💰 Per-vector billing |
| Cosine similarity | ✅ `<=>` operator | ✅ Yes | ✅ Yes |

**Key insight**: Our hybrid search merges `tsvector` keyword scores and `pgvector` cosine similarity scores via RRF in a single SQL query. This is only possible because both live in PostgreSQL. A dedicated vector DB would require two separate queries and application-level score fusion.

---

## Embedding Tables

```sql
repository_embeddings  -- repository-level vectors
project_embeddings     -- project description vectors
document_embeddings    -- README / markdown vectors
code_embeddings        -- code snippet vectors
resume_embeddings      -- resume section vectors
skill_embeddings       -- skill name/description vectors
```

---

## Consequences

- All embedding tables must use `vector(1536)` column type (or dimension-configurable).
- IVFFlat index created for each embedding table: `CREATE INDEX ON <table> USING ivfflat (embedding vector_cosine_ops)`.
- Hybrid search combines `tsvector @@ to_tsquery(...)` with `embedding <=> $1` in one query using Reciprocal Rank Fusion.
- **Do not introduce Qdrant, Weaviate, Milvus, or Pinecone.** All vector operations stay in PostgreSQL.
- Vector dimension is controlled by `AI__EMBEDDING_DIMENSIONS` environment variable. Default: 1536.
