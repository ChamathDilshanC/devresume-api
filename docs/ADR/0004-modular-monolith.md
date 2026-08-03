# ADR 0004 — Modular Monolith over Microservices

- **Status**: Accepted
- **Date**: 2026-01-01
- **Author**: ChamathDilshanC

---

## Context

Early-stage SaaS products face a common architectural decision: start with a monolith (fast to build, hard to scale later) or microservices (complex infrastructure, organizational overhead). A third option — the modular monolith — provides domain separation without distributed systems complexity.

---

## Decision

**Use a Rust workspace-based modular monolith as the foundational architecture.**

---

## Definition

A **modular monolith** is a single deployable unit where:
- Each domain is an independent Rust crate with strict dependency rules.
- Crates communicate via function calls, not network calls.
- The workspace enforces dependency boundaries at compile time.
- Each crate can be extracted into a microservice with minimal code changes when needed.

---

## Structure

```
devresume-api/
├── apps/api/          # Single binary — the monolith entrypoint
├── apps/worker/       # Worker binary (can scale independently)
├── crates/auth/       # Domain crate — no knowledge of HTTP layer
├── crates/github/     # Domain crate — no knowledge of resume crate
├── crates/resume/     # Domain crate — no knowledge of github crate
└── shared/            # Zero business logic — only utilities
```

---

## Dependency Rules (Enforced by Cargo)

```
apps/api → crates/* (allowed — composes domains)
apps/worker → crates/* (allowed — processes jobs)
crates/github → shared (allowed)
crates/resume → shared (allowed)
crates/github → crates/resume (FORBIDDEN)
crates/resume → crates/github (FORBIDDEN)
crates/ai → PostgreSQL (FORBIDDEN — AI crate must not touch DB)
shared → any domain crate (FORBIDDEN)
```

---

## Rationale

| Criterion | Modular Monolith | Microservices |
|-----------|-----------------|--------------|
| Operational complexity | ✅ Low | ❌ High |
| Team size fit | ✅ Solo/small team | ❌ Multiple teams |
| Refactoring cost | ✅ Compile-time errors | ❌ Breaking APIs |
| Domain isolation | ✅ Crate boundaries | ✅ Service boundaries |
| Deployment | ✅ Single container | ❌ Orchestration |
| Future extraction | ✅ Straightforward | N/A |
| Distributed tracing | ✅ Inprocess | ❌ Required from day 1 |

---

## Microservice Extraction Path

When a domain requires independent scaling (e.g., the `ai` crate hits compute limits):

1. Extract crate to a new repository.
2. Replace direct function call with an HTTP/gRPC client.
3. No business logic changes required — only the transport layer changes.

---

## Consequences

- **Do not introduce service-to-service HTTP calls within the workspace.**
- **Do not add message queue dependencies between crates** (use the `job_queue` table instead).
- All new features must be added as domain crates, not mixed into `apps/api/`.
- The workspace has a single `Cargo.lock` — dependency versions are shared.
