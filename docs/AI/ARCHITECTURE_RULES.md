# DevResume AI — Architecture Rules

These rules define the dependency boundaries enforced in the Rust workspace. They must be respected by all AI agents and human contributors.

**Violation of these rules is a build error — Cargo will refuse to compile if dependencies cross the stated boundaries.**

---

## Dependency Graph (Allowed →)

```
apps/api
  → crates/auth
  → crates/github
  → crates/parser
  → crates/ai
  → crates/resume
  → crates/portfolio
  → crates/ats
  → crates/analytics
  → crates/search
  → crates/notification
  → crates/career
  → crates/jobs
  → crates/interview
  → crates/learning
  → crates/recommendation
  → crates/storage
  → shared/*

apps/worker
  → crates/* (same as api)
  → shared/*

apps/cli
  → crates/auth
  → crates/github
  → shared/*

crates/* (any domain crate)
  → shared/*
  → crates/common (config, models, error types)

shared/*
  → external dependencies ONLY (no domain crates)
```

---

## FORBIDDEN Dependencies

These cross-domain dependencies break domain isolation and are **strictly prohibited**:

### Rule 1 — API Layer Cannot Access DB Directly

```
❌ apps/api → sqlx (direct)
✓  apps/api → crates/auth → sqlx (via repository)
```

The API layer must go through domain crate repositories. No raw SQL in route handlers.

---

### Rule 2 — Domain Cannot Depend on Web Layer

```
❌ crates/resume → axum
❌ crates/auth → axum::Router
✓  crates/resume → (pure domain, no HTTP)
✓  apps/api → crates/resume → (wires HTTP to domain)
```

Domain crates must be HTTP-agnostic. They contain service logic only.

---

### Rule 3 — Resume Cannot Access GitHub Directly

```
❌ crates/resume → crates/github
✓  apps/api/handlers/resume → crates/github (coordinator)
✓  apps/api/handlers/resume → crates/resume
```

The GitHub crate fetches raw data. The Resume crate transforms it into documents. Neither should know about the other — the API layer orchestrates.

---

### Rule 4 — AI Cannot Access PostgreSQL Directly

```
❌ crates/ai → sqlx
❌ crates/ai → crates/common::Database
✓  crates/ai → (pure computation — no persistence)
✓  apps/worker/embedding_worker → crates/ai + sqlx (stores embeddings)
```

The AI crate is a computation layer only: it receives text, returns embeddings or generated text. It must not own a database connection.

---

### Rule 5 — Shared Crates Have Zero Business Logic

```
❌ shared/pagination → crates/auth (any import)
❌ shared/pagination → business rules of any kind
✓  shared/pagination → (pure utility: structs, traits, helpers)
✓  shared/errors → (generic error types, no domain errors)
```

Shared crates are infrastructure utilities. Domain logic must never live in `shared/`.

---

### Rule 6 — Parser Cannot Access AI Directly

```
❌ crates/parser → crates/ai
✓  crates/parser → (pure parsing — deterministic output)
✓  apps/worker/embedding_worker → crates/parser + crates/ai
```

Parsing is deterministic and fast. AI inference is non-deterministic and slow. They must be separate stages, composed in workers.

---

### Rule 7 — Notification Cannot Access Domain Business Logic

```
❌ crates/notification → crates/resume
❌ crates/notification → crates/career
✓  crates/notification → (dispatches pre-built messages only)
```

The notification crate receives formatted messages from the job queue and dispatches them. It does not know what triggered the notification.

---

## Verification Checklist (Before Each Module Commit)

Run this to verify no forbidden dependencies crept in:

```bash
# Check dependency tree for a crate
cargo tree -p devresume-resume --depth 2

# Check all workspace crate dependencies
cargo tree --workspace --depth 2

# Ensure no axum dependency in domain crates
cargo tree -p devresume-resume | grep axum
cargo tree -p devresume-auth | grep sqlx   # should only be via trait
cargo tree -p devresume-ai | grep sqlx     # should return empty
```

---

## How to Enforce at Compile Time

Add to each domain crate's `Cargo.toml` dev-dependencies the `cargo-deny` configuration:

```toml
# In workspace Cargo.toml
[workspace.metadata.cargo-deny]
# Deny unknown licenses
[bans]
multiple-versions = "warn"
```

In CI:

```bash
cargo deny check bans
cargo deny check licenses
```

---

## Allowed External Dependencies Per Layer

| Layer | Database | HTTP | Async | AI SDK |
|-------|----------|------|-------|--------|
| `apps/api` | Via crates only | ✅ axum | ✅ tokio | ❌ |
| `apps/worker` | Via crates only | ❌ | ✅ tokio | ❌ |
| `crates/auth` | ✅ sqlx | ❌ | ✅ tokio | ❌ |
| `crates/ai` | ❌ | ✅ reqwest | ✅ tokio | ✅ openai-rs etc |
| `crates/parser` | ❌ | ❌ | ❌ | ❌ |
| `crates/resume` | ❌ | ❌ | ✅ tokio | ❌ |
| `shared/*` | ❌ | ❌ | ⚠️ minimal | ❌ |

---

## Summary — Quick Reference

```
Rule 1: API → DB must go via repository (no direct sqlx in handlers)
Rule 2: Domain crates must not import axum
Rule 3: Resume crate must not import github crate
Rule 4: AI crate must not import sqlx
Rule 5: Shared crates must have zero business logic
Rule 6: Parser crate must not import AI crate
Rule 7: Notification crate must not import domain crates
```
