# DevResume AI — Module Implementation Checklist

Use this checklist for every module. A module cannot be committed until all items are checked.

---

## Pre-Implementation

- [ ] Read `ARCHITECTURE.md` — understand the domain boundaries
- [ ] Read `docs/DOMAIN_MODEL.md` — understand the entity relationships
- [ ] Check existing crates for overlapping functionality
- [ ] Check existing SQL migrations for required schema
- [ ] Check existing API routes for duplicate endpoints
- [ ] Confirm module placement in the correct crate

---

## Phase 1 — 🔐 Authentication & User Management

- [ ] `User` model with all required fields
- [ ] `Account` model for OAuth providers
- [ ] `Session` model for active sessions
- [ ] `UserRepository` with CRUD + soft delete
- [ ] Password hashing with Argon2
- [ ] JWT access token generation (7-day expiry)
- [ ] Refresh token generation and rotation
- [ ] GitHub OAuth flow (authorization → callback → token exchange)
- [ ] Google OAuth flow (authorization → callback → token exchange)
- [ ] RBAC: roles `admin`, `developer`, `recruiter`
- [ ] Permission checks as Axum middleware
- [ ] API key generation and validation
- [ ] Rate limiting on `/api/v1/auth/*`
- [ ] Email verification flow
- [ ] Password reset flow
- [ ] `POST /api/v1/auth/login`
- [ ] `POST /api/v1/auth/register`
- [ ] `POST /api/v1/auth/refresh`
- [ ] `POST /api/v1/auth/logout`
- [ ] `GET  /api/v1/auth/github`
- [ ] `GET  /api/v1/auth/github/callback`
- [ ] `GET  /api/v1/auth/google`
- [ ] `GET  /api/v1/auth/google/callback`
- [ ] Unit tests for JWT logic
- [ ] Unit tests for password hashing
- [ ] Integration tests for login flow
- [ ] Integration tests for OAuth callback

---

## Phase 2 — 🐙 GitHub Integration

- [ ] GitHub API client (reqwest, auth header injection)
- [ ] Repository listing from GitHub API
- [ ] Repository sync to local database
- [ ] Webhook signature verification (HMAC-SHA256)
- [ ] Webhook event handling (push, create, delete)
- [ ] Commit sync with diff stats
- [ ] Issue sync
- [ ] Pull Request sync
- [ ] Release sync
- [ ] Branch tracking
- [ ] Language detection from GitHub API
- [ ] README extraction and storage
- [ ] License detection
- [ ] Contributor detection
- [ ] Incremental sync (only changed repos)
- [ ] `GET  /api/v1/repositories`
- [ ] `POST /api/v1/repositories/sync`
- [ ] `GET  /api/v1/repositories/:id`
- [ ] `GET  /api/v1/repositories/:id/commits`
- [ ] `POST /api/v1/webhooks/github`
- [ ] Unit tests for webhook signature
- [ ] Integration tests for sync flow

---

## Phase 3 — 📂 Parser & Repository Scanner

- [ ] `Cargo.toml` parser — detects Rust deps
- [ ] `package.json` parser — detects JS/TS deps
- [ ] `pom.xml` parser — detects Java deps
- [ ] `build.gradle` parser — detects Gradle deps
- [ ] `Dockerfile` parser — detects base images, exposed ports
- [ ] `docker-compose.yml` parser — detects services
- [ ] GitHub Actions parser — detects CI/CD workflows
- [ ] Kubernetes YAML parser
- [ ] Terraform HCL parser
- [ ] Helm chart parser
- [ ] README.md semantic extraction
- [ ] Language aggregation from file extensions
- [ ] Framework detection from dependency manifests
- [ ] Database detection (postgres, mysql, redis, mongo)
- [ ] Cloud provider detection (AWS, GCP, Azure)
- [ ] Architecture pattern detection (microservices, monolith, serverless)
- [ ] Unit tests for each parser

---

## Phase 4 — 🤖 AI Engine

- [ ] `AIProvider` trait with `generate`, `embeddings`, `chat`
- [ ] `OpenAIProvider` implementation
- [ ] `GeminiProvider` implementation
- [ ] `ClaudeProvider` implementation
- [ ] `OllamaProvider` implementation
- [ ] Dynamic provider selection from config
- [ ] Prompt template engine
- [ ] Text chunking strategy (by token count)
- [ ] Embedding generation for all 6 vector tables
- [ ] Retry logic with exponential backoff
- [ ] Token usage tracking per request
- [ ] Structured JSON output parsing
- [ ] `POST /api/v2/ai/generate`
- [ ] `POST /api/v2/ai/embed`
- [ ] Unit tests for prompt builder
- [ ] Unit tests for chunking
- [ ] Integration tests for provider abstraction

---

## Phase 5 — 📄 Resume Engine

- [ ] Resume JSON builder from user profile
- [ ] Resume Markdown renderer
- [ ] Resume HTML renderer
- [ ] Resume PDF compiler (Typst or Headless Chrome)
- [ ] Resume DOCX generator
- [ ] ATS-optimized resume variant
- [ ] Modern resume template
- [ ] Executive resume template
- [ ] Resume version history (create on each generate)
- [ ] `POST /api/v1/resumes/generate`
- [ ] `GET  /api/v1/resumes`
- [ ] `GET  /api/v1/resumes/:id`
- [ ] `GET  /api/v1/resumes/:id/download?format=pdf`
- [ ] Unit tests for JSON builder
- [ ] Integration tests for export formats

---

## Phase 6 — 🌐 Portfolio Engine

- [ ] Portfolio config JSON builder
- [ ] Project section aggregation
- [ ] Skills section aggregation
- [ ] Career timeline events
- [ ] GitHub stats integration
- [ ] Custom domain support in portfolio table
- [ ] `GET  /api/v1/portfolio`
- [ ] `PUT  /api/v1/portfolio`
- [ ] `POST /api/v1/portfolio/publish`
- [ ] Unit tests for portfolio builder

---

## Phase 7 — 📊 ATS & Analytics

**ATS:**
- [ ] Keyword density analysis
- [ ] ATS overall score (0–100)
- [ ] Readability score (Flesch–Kincaid)
- [ ] Action verb detection
- [ ] Duplicate phrase detection
- [ ] Improvement suggestions list
- [ ] `POST /api/v1/ats/score`
- [ ] `GET  /api/v1/ats/reports`

**Analytics:**
- [ ] Technology usage stats per user
- [ ] Commit count & velocity over time
- [ ] Repository contribution heatmap data
- [ ] Career progress score
- [ ] Contribution score calculator
- [ ] Skill growth timeline
- [ ] Project impact score
- [ ] Dashboard summary endpoint
- [ ] `GET /api/v1/analytics/overview`
- [ ] `GET /api/v1/analytics/technologies`
- [ ] `GET /api/v2/analytics/career`

---

## Phase 8 — 🔍 Hybrid Search

- [ ] Full-text search using `tsvector` / `to_tsquery`
- [ ] Vector cosine similarity search using `pgvector`
- [ ] Reciprocal Rank Fusion (RRF) score merger
- [ ] Search across: projects, repositories, skills, resume content
- [ ] Configurable result limit & offset pagination
- [ ] `POST /api/v2/search`
- [ ] `POST /api/v2/search/vector`
- [ ] `POST /api/v2/search/hybrid`
- [ ] Unit tests for RRF score calculation
- [ ] Integration tests for search results

---

## Phase 9 — 🔔 Notifications & Workers

**Notifications:**
- [ ] Email notification via SMTP
- [ ] In-app notification creation
- [ ] Weekly digest email
- [ ] Webhook notification dispatch
- [ ] Notification templates engine
- [ ] `GET  /api/v1/notifications`
- [ ] `PUT  /api/v1/notifications/:id/read`

**Workers:**
- [ ] `sync_worker` — polls GitHub sync queue
- [ ] `github_worker` — processes webhook events
- [ ] `resume_worker` — compiles PDF/DOCX artifacts
- [ ] `portfolio_worker` — renders portfolio data
- [ ] `embedding_worker` — generates pgvector embeddings
- [ ] `notification_worker` — dispatches email and in-app
- [ ] `cleanup_worker` — expires tokens, removes dead jobs
- [ ] `scheduler_worker` — cron-based periodic tasks
- [ ] Dead Letter Queue (DLQ) processing
- [ ] Retry with exponential backoff

---

## Phase 10 — 🚀 Production Hardening

- [ ] Prometheus metrics endpoint (`/metrics`)
- [ ] OpenTelemetry tracing setup
- [ ] Health check (`GET /health`)
- [ ] Readiness check (`GET /ready`)
- [ ] Liveness check (`GET /live`)
- [ ] CORS policy enforcement
- [ ] Security response headers
- [ ] Request ID / correlation ID middleware
- [ ] Global rate limiting middleware
- [ ] Connection pool tuning
- [ ] OpenAPI spec generation
- [ ] `cargo clippy -- -D warnings` clean
- [ ] Full `cargo test` suite passes
- [ ] Docker Compose verified

---

## Definition of Done — Per Module

A module is considered **DONE** only if ALL of the following are true:

- [ ] ✓ Compiles without errors or warnings
- [ ] ✓ All tests pass (`cargo test`)
- [ ] ✓ All public functions have unit tests
- [ ] ✓ All API endpoints have integration tests
- [ ] ✓ SQL migration exists for schema changes
- [ ] ✓ Structured logging implemented (`tracing::info!`)
- [ ] ✓ Input validation implemented at API boundary
- [ ] ✓ Typed error handling via `thiserror`
- [ ] ✓ HTTP error mapping via `IntoResponse`
- [ ] ✓ OpenAPI annotations added
- [ ] ✓ `docs/` updated if API surface changed
- [ ] ✓ No `.unwrap()` or `.expect()` in production paths
- [ ] ✓ No hardcoded secrets
- [ ] ✓ `cargo fmt` passes
- [ ] ✓ `cargo clippy -- -D warnings` passes
- [ ] ✓ Committed with correct author
