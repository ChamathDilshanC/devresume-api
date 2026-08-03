# DevResume AI — Architecture Overview

> Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)

Full architecture documentation is available in the root `ARCHITECTURE.md`. This file serves as the docs-directory copy for AI agent reference.

Please refer to [`ARCHITECTURE.md`](../../ARCHITECTURE.md) at the repository root for the full system design.

---

## Quick Reference: Key Components

| Layer | Technology | Crate |
|-------|-----------|-------|
| HTTP Server | Axum | `apps/api` |
| Background Jobs | Tokio tasks | `apps/worker` |
| Admin CLI | Clap | `apps/cli` |
| Auth & JWT | jsonwebtoken, argon2 | `crates/auth` |
| GitHub Integration | reqwest | `crates/github` |
| Parser | Custom parsers | `crates/parser` |
| AI Engine | OpenAI / Gemini / Claude / Ollama | `crates/ai` |
| Resume Engine | Typst, pandoc | `crates/resume` |
| Portfolio | JSON rendering | `crates/portfolio` |
| ATS Scoring | Rust custom | `crates/ats` |
| Analytics | Rust custom | `crates/analytics` |
| Search | pgvector + tsvector | `crates/search` |
| Storage | MinIO / S3 | `crates/storage` |
| Notifications | SMTP + in-app | `crates/notification` |
| Career | Goals, timeline | `crates/career` |
| Jobs | Applications tracker | `crates/jobs` |
| Interview | AI question gen | `crates/interview` |
| Learning | Skill recommendations | `crates/learning` |
| Recommendation | Career insights | `crates/recommendation` |
| Shared Utilities | Pagination, helpers | `shared/` |
| Config & Models | Config, DB pool, models | `crates/common` |

---

## Enterprise Directory Layout

```
devresume-api/
├── apps/
│   ├── api/            # Axum HTTP server
│   ├── worker/         # Background workers
│   └── cli/            # Admin CLI
├── crates/             # Domain micro-crates (17 total)
├── shared/             # Shared utilities & pagination
├── migrations/         # SQLx PostgreSQL migrations
├── configs/            # YAML environment configurations
├── fixtures/           # Test data fixtures
├── scripts/            # Seed & utility scripts
├── tests/              # Integration test suite
├── examples/           # API usage examples
└── docs/
    ├── ARCHITECTURE.md
    ├── DATABASE.md
    ├── API_SPEC.md
    ├── DOMAIN_MODEL.md
    ├── FEATURES.md
    ├── ROADMAP.md
    ├── SECURITY.md
    ├── DEPLOYMENT.md
    ├── CODING_STANDARDS.md
    ├── CONTRIBUTING.md
    └── AI/
        ├── AGENT_PROMPT.md
        ├── DEVELOPMENT_RULES.md
        ├── CODING_GUIDELINES.md
        ├── MODULE_CHECKLIST.md
        ├── API_CHECKLIST.md
        ├── TESTING_GUIDE.md
        └── DONE_DEFINITION.md
```
