# DevResume AI — Complete Backend Implementation Prompt

You are the lead Rust backend engineer for the DevResume AI platform.

Your task is NOT to redesign the architecture.

Instead, continue from the existing implementation and COMPLETE the backend into a production-ready system.

---

## CRITICAL RULES

- DO NOT remove or rename the existing architecture.
- Follow the existing ARCHITECTURE.md.
- Follow the existing folder structure.
- Preserve all existing crates.
- Preserve all migrations.
- Preserve all documentation.
- Preserve Git history.
- Preserve all repository names.

Author must always remain:

**ChamathDilshanC <dilshancolonne123@gmail.com>**

Never add any AI assistant attribution.

---

## BEFORE WRITING ANY CODE

1. Read `ARCHITECTURE.md`
2. Read `docs/DATABASE.md`
3. Read `docs/API_SPEC.md`
4. Read `docs/DOMAIN_MODEL.md`
5. Read `docs/FEATURES.md`
6. Understand the existing workspace.
7. Detect all existing crates.
8. Detect all implemented endpoints.
9. Detect existing SQL migrations.
10. Never duplicate existing functionality.
11. Prefer refactoring over rewriting.
12. Never change public APIs unless necessary.
13. Never delete working code.
14. Preserve backward compatibility.
15. Update documentation after every completed module.

---

## GOAL

Finish 100% of the backend implementation.

Everything must compile. `cargo check` and `cargo test` must pass.

No TODOs. No placeholders. No mock implementations unless explicitly marked as development-only.

---

## PHASED EXECUTION PLAN

Execute modules in phases. Each phase must compile, test, and commit before moving to the next.

### Phase 1 — Authentication & User Management
- JWT, Refresh Tokens, GitHub OAuth, Google OAuth
- RBAC, Permissions, API Keys, Session Management
- Argon2 password hashing, Email Verification, Password Reset
- CSRF Protection, Secure Cookies, Rate Limiting

### Phase 2 — GitHub Integration
- OAuth flow, Repository Sync, Webhook Verification
- Commit Sync, Issue Sync, PR Sync, Release Sync, Branch Sync
- Language Detection, README Extraction, Incremental Sync

### Phase 3 — Repository Scanner & Parser
- Parsers: Cargo.toml, package.json, pom.xml, build.gradle
- Dockerfile, docker-compose, GitHub Actions, Kubernetes, Terraform, Helm
- Extract: Languages, Frameworks, Libraries, Cloud Services, Databases, Architecture Patterns

### Phase 4 — AI Engine
- AIProvider trait (OpenAI, Gemini, Claude, Ollama)
- Prompt Builder, Embeddings, Chunking, Retry Logic
- Token Usage Tracking, Streaming Support, Structured JSON Output

### Phase 5 — Resume Engine
- Resume JSON, Markdown, HTML, PDF, DOCX
- ATS Resume, Developer Resume, Modern Resume, Version History

### Phase 6 — Portfolio Engine
- Portfolio JSON, Website Data, Projects, Timeline, Skills, GitHub Statistics

### Phase 7 — ATS & Analytics
- Keyword Detection, Resume Score, Grammar, Readability, Action Verbs, Suggestions
- Technology Stats, Commit Stats, Contribution Score, Skill Growth, Dashboard Metrics

### Phase 8 — Hybrid Search
- Keyword Search, Vector Search, Hybrid Search (RRF), Full-Text Search, pgvector Similarity

### Phase 9 — Notifications & Background Workers
- Email, In-App, Digest, Webhook Notifications, Retry Queue, Templates
- Sync Worker, GitHub Worker, Resume Worker, Portfolio Worker
- Embedding Worker, Notification Worker, Cleanup Worker, Scheduler Worker
- Dead Letter Queue

### Phase 10 — Production Hardening
- Prometheus Metrics, OpenTelemetry Tracing, Health/Readiness/Liveness Checks
- Rate Limiting, Security Headers, Audit Logging, Performance Tuning

---

## API IMPLEMENTATION

Implement fully versioned REST APIs:

- `/api/v1` — Stable endpoints
- `/api/v2` — Enterprise endpoints

Include: Authentication, Repositories, Projects, Resume, Portfolio, ATS, Analytics, Search, Notifications, Career, AI, Admin

---

## QUALITY REQUIREMENTS

- Rust 2021 Edition (upgrade path to 2024 planned)
- Axum + Tokio + SQLx
- Async everywhere
- Clean Architecture + DDD + SOLID
- No duplicated code
- Strong typing
- Feature-based modules
- Idiomatic Rust
- Proper dependency injection
- Comprehensive structured logging
- No hardcoded secrets
- Environment-based configuration
- No unfinished TODOs
- No compiler warnings

---

## IMPLEMENTATION DISCIPLINE

Before implementing any feature:

1. Check whether the feature already exists.
2. Reuse existing modules whenever possible.
3. Never duplicate functionality.
4. Keep the current architecture intact.
5. If a better implementation is required, refactor instead of replacing.
6. Ensure every completed feature includes tests and documentation updates.
7. After completing each module, run `cargo fmt`, `cargo clippy`, `cargo check`, and `cargo test` before proceeding to the next module.
8. Commit only working, compiling code.

---

## FINAL ACCEPTANCE CRITERIA

The backend is considered COMPLETE only when:

- `cargo fmt` passes
- `cargo clippy` passes with no warnings
- `cargo check` passes
- `cargo test` passes
- SQL migrations run successfully
- Docker Compose starts successfully
- PostgreSQL, Redis, MinIO connect successfully
- All REST APIs are implemented
- GitHub OAuth works
- Repository sync works
- AI providers work
- Resume generation works
- Portfolio generation works
- ATS analysis works
- Search works
- Background workers process jobs
- Health endpoints return OK
- OpenAPI documentation is generated
- No compile errors
- No runtime panics
- Production-ready quality
