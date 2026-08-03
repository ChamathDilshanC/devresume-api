# DevResume AI — Roadmap

> Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)

---

## Milestone 1 — 🔐 Authentication & User Management

**Goal**: Secure user identity and access control foundation.

**Features**:
- Email/Password registration and login
- GitHub OAuth + Google OAuth
- JWT access + Refresh token rotation
- RBAC (admin / developer / recruiter)
- API key management
- Rate limiting on auth endpoints

**Done when**: All auth flows work. `cargo test` passes. OAuth works end-to-end.

---

## Milestone 2 — 🐙 GitHub Integration

**Goal**: Connect GitHub account and sync repository metadata.

**Features**:
- GitHub repository listing
- Full + incremental sync
- Commit, PR, Issue, Release sync
- Webhook event processing
- Language and technology detection

**Done when**: Repository sync completes. Webhook verified. Incremental sync tested.

---

## Milestone 3 — 📂 Repository Scanner & Parser

**Goal**: Deep code analysis to detect technology stack automatically.

**Features**:
- Cargo.toml, package.json, pom.xml, build.gradle parsers
- Dockerfile, docker-compose, GitHub Actions, Kubernetes parsers
- Language + Framework + Database + Cloud + Architecture detection

**Done when**: Parser produces accurate technology profiles from real repos.

---

## Milestone 4 — 🤖 AI Engine

**Goal**: Pluggable multi-provider AI layer for all generative tasks.

**Features**:
- AIProvider trait (OpenAI, Gemini, Claude, Ollama)
- Prompt builder, embeddings, chunking
- Retry logic, token usage tracking, structured JSON output

**Done when**: At least one provider works. Embeddings stored in pgvector.

---

## Milestone 5 — 📄 Resume Engine

**Goal**: Generate professional resumes from profile data.

**Features**:
- JSON, Markdown, HTML, PDF, DOCX export
- ATS-optimized variant
- Modern + Executive templates
- Version history

**Done when**: PDF resume generated from test user data. All formats downloadable.

---

## Milestone 6 — 🌐 Portfolio Engine

**Goal**: Auto-generate a developer portfolio from synced data.

**Features**:
- Portfolio JSON builder
- Projects, Skills, Timeline, Stats sections
- Publish to subdomain / custom domain

**Done when**: Portfolio JSON correct. Publish endpoint returns 200.

---

## Milestone 7 — 📊 ATS & Analytics

**Goal**: Score resumes and provide career analytics.

**Features**:
- ATS score (0–100) with suggestions
- Technology usage stats
- Contribution score + Skill growth

**Done when**: ATS score endpoint returns valid structured output.

---

## Milestone 8 — 🔍 Hybrid Search

**Goal**: Intelligent search across all user content.

**Features**:
- Full-text search (tsvector)
- Vector similarity (pgvector)
- Hybrid (RRF fusion)

**Done when**: `/api/v2/search/hybrid` returns ranked, relevant results.

---

## Milestone 9 — 🔔 Notifications & Background Workers

**Goal**: Async event processing and user communication.

**Features**:
- 8 background workers (sync, github, resume, portfolio, embedding, notification, cleanup, scheduler)
- Email + in-app + digest notifications
- Dead Letter Queue

**Done when**: All workers process queued jobs. Email delivered in staging.

---

## Milestone 10 — 🚀 Production Hardening

**Goal**: Production-grade observability, security, and performance.

**Features**:
- Prometheus metrics, OpenTelemetry tracing
- Health/Readiness/Liveness endpoints
- CORS, security headers, rate limiting
- OpenAPI generation
- Full `cargo test` suite (≥ 80% coverage)

**Done when**: All CI gates pass. Docker Compose verified. Load test clean.
