# ADR 0001 — Use Rust + Axum as HTTP Framework

- **Status**: Accepted
- **Date**: 2026-01-01
- **Author**: ChamathDilshanC

---

## Context

The DevResume AI backend requires a high-throughput, low-latency HTTP server capable of handling concurrent GitHub sync operations, AI embedding pipelines, and real-time analytics queries. The server must be production-ready with first-class async support, type safety, and minimal runtime overhead.

Candidates evaluated:
- **Axum** (Tokio ecosystem)
- **Actix-web** (actor model)
- **Warp** (filter-chain based)
- **Rocket** (macro-heavy, async experimental)

---

## Decision

**Use Axum with Tokio as the HTTP framework.**

---

## Rationale

| Criterion | Axum | Actix-web | Warp |
|-----------|------|-----------|------|
| Tokio-native | ✅ First-class | ⚠️ Own runtime | ✅ Yes |
| Ergonomics | ✅ Extractor-based | ⚠️ Verbose | ⚠️ Closure-chain |
| Tower middleware | ✅ Full compatibility | ❌ No | ✅ Partial |
| Type-safe routing | ✅ Yes | ✅ Yes | ✅ Yes |
| Error handling | ✅ IntoResponse | ✅ ResponseError | ⚠️ Rejection |
| Maintenance | ✅ Tokio team | ✅ Active | ⚠️ Slower |
| Ecosystem fit | ✅ Tower + Hyper | ❌ Diverges | ⚠️ Partial |

Axum's extractor pattern aligns with our DDD architecture — request parsing, auth, and database access all compose cleanly via `FromRequestParts`. Tower middleware gives us rate limiting, tracing, and CORS without custom code.

---

## Consequences

- All HTTP handlers must use Axum extractors (`State`, `Json`, `Path`, `Extension`).
- Middleware must be Tower-compatible (`tower::Layer`).
- No mixing of Actix-web or other HTTP frameworks in this workspace.
- Error types must implement `IntoResponse`.
- This decision is **final** — do not introduce alternative HTTP crates.
