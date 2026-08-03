# DevResume AI — Complete Backend Implementation Prompt

You are the lead Rust backend engineer for the DevResume AI platform.

Your task is NOT to redesign the architecture.

Instead, continue from the existing implementation and COMPLETE the backend into a production-ready system.

IMPORTANT:

- DO NOT remove or rename the existing architecture.
- Follow the existing ARCHITECTURE.md.
- Follow the existing folder structure.
- Preserve all existing crates.
- Preserve all migrations.
- Preserve all documentation.
- Preserve Git history.
- Preserve all repository names.

Author must always remain

ChamathDilshanC <dilshancolonne123@gmail.com>

Never add any AI assistant attribution.

====================================================
GOAL
====================================================

Finish 100% of the backend implementation.

Everything should compile successfully.

cargo check

cargo test

must pass.

No TODOs.

No placeholders.

No mock implementations unless explicitly marked as development-only.

====================================================
IMPLEMENT ALL DOMAIN MODULES
====================================================

Complete implementation for

- Authentication
- Users
- OAuth
- GitHub Integration
- Repository Sync
- Repository Scanner
- Parser
- AI Engine
- Resume Engine
- Portfolio Engine
- ATS Engine
- Analytics
- Career Timeline
- Recommendation Engine
- Interview Engine
- Learning Engine
- Notification Engine
- Search Engine
- Storage
- Background Workers
- Admin CLI

====================================================
AUTHENTICATION
====================================================

Implement

- JWT
- Refresh Tokens
- GitHub OAuth
- Google OAuth
- RBAC
- Permissions
- API Keys
- Session Management
- Password Hashing
- Email Verification
- Password Reset

Security

- Argon2
- CSRF Protection
- Secure Cookies
- Rate Limiting

====================================================
DATABASE
====================================================

Complete SQLx implementation.

Implement

Repositories

Models

Queries

Transactions

Indexes

Constraints

Relationships

Pagination

Filtering

Sorting

Soft Delete

Audit Logging

Connection Pool

====================================================
GITHUB
====================================================

Implement

GitHub OAuth

Repository Sync

Webhook Verification

Repository Clone

Commit Sync

Issue Sync

PR Sync

Release Sync

Branch Sync

Language Detection

README Extraction

License Detection

Contributor Detection

Repository Statistics

Incremental Sync

====================================================
PARSER
====================================================

Implement parsers for

Cargo.toml

package.json

pom.xml

build.gradle

Dockerfile

docker-compose

GitHub Actions

Kubernetes

Terraform

Helm

README

Markdown

Source Code

Extract

Languages

Frameworks

Libraries

Cloud Services

Databases

Architecture Patterns

====================================================
AI ENGINE
====================================================

Implement

AIProvider trait

Providers

OpenAI

Gemini

Claude

Ollama

Prompt Builder

Embeddings

Chunking

Retry Logic

Token Usage Tracking

Streaming Support

Structured JSON Output

====================================================
RESUME ENGINE
====================================================

Generate

Resume JSON

Resume Markdown

Resume HTML

Resume PDF

Resume DOCX

ATS Resume

Developer Resume

Modern Resume

Resume Version History

====================================================
PORTFOLIO ENGINE
====================================================

Generate

Portfolio JSON

Portfolio Website Data

Projects

Timeline

Skills

Experience

GitHub Statistics

====================================================
ATS ENGINE
====================================================

Implement

Keyword Detection

Resume Score

Grammar Checks

Readability

Action Verb Detection

Duplicate Detection

Improvement Suggestions

====================================================
SEARCH
====================================================

Implement

Keyword Search

Vector Search

Hybrid Search

Full Text Search

pgvector Similarity

Ranking

Filtering

====================================================
ANALYTICS
====================================================

Implement

Technology Statistics

Commit Statistics

Repository Metrics

Career Progress

Contribution Score

Skill Growth

Project Impact Score

Dashboard Metrics

====================================================
NOTIFICATIONS
====================================================

Implement

Email

In-App

Digest

Webhook Notifications

Retry Queue

Templates

====================================================
BACKGROUND WORKERS
====================================================

Implement

Sync Worker

GitHub Worker

Resume Worker

Portfolio Worker

Embedding Worker

Notification Worker

Cleanup Worker

Scheduler Worker

Retry Logic

Dead Letter Queue

====================================================
API
====================================================

Implement fully versioned REST APIs

/api/v1

/api/v2

Include

Authentication

Repositories

Projects

Resume

Portfolio

ATS

Analytics

Search

Notifications

Career

AI

Admin

====================================================
VALIDATION
====================================================

Validate

Request DTOs

Response DTOs

Business Rules

Unique Constraints

Permissions

====================================================
ERROR HANDLING
====================================================

Implement

Typed Errors

HTTP Error Mapping

Tracing

Structured Logging

Correlation IDs

====================================================
OBSERVABILITY
====================================================

Implement

Prometheus Metrics

Tracing

Health Checks

Readiness Checks

Liveness Checks

OpenTelemetry

====================================================
TESTING
====================================================

Write

Unit Tests

Integration Tests

Repository Tests

API Tests

Worker Tests

Authentication Tests

Minimum coverage target

80%

====================================================
DOCUMENTATION
====================================================

Update

README.md

API_SPEC.md

DATABASE.md

SECURITY.md

DEPLOYMENT.md

CHANGELOG.md

Generate OpenAPI documentation.

====================================================
QUALITY REQUIREMENTS
====================================================

- Rust 2024 Edition
- Axum
- Tokio
- SQLx
- Async Everywhere
- Clean Architecture
- DDD
- SOLID
- No duplicated code
- Strong typing
- Feature-based modules
- Idiomatic Rust
- Production-ready code
- Proper dependency injection
- Comprehensive logging
- No hardcoded secrets
- Environment-based configuration
- No unfinished TODOs

====================================================
FINAL ACCEPTANCE CRITERIA
====================================================

The backend is considered COMPLETE only when:

- cargo fmt passes
- cargo clippy passes with no warnings
- cargo check passes
- cargo test passes
- SQL migrations run successfully
- Docker Compose starts successfully
- PostgreSQL connects successfully
- Redis connects successfully
- MinIO connects successfully
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

====================================================
IMPLEMENTATION DISCIPLINE
====================================================

Before implementing any feature:

1. Check whether the feature already exists.
2. Reuse existing modules whenever possible.
3. Never duplicate functionality.
4. Keep the current architecture intact.
5. If a better implementation is required, refactor instead of replacing.
6. Ensure every completed feature includes tests and documentation updates.
7. After completing each module, run cargo fmt, cargo clippy, cargo check, and cargo test before proceeding to the next module.
8. Commit only working, compiling code.
