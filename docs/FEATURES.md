# DevResume AI — Feature Specifications

> Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)

This document lists every product feature with its implementation scope and status.

---

## Feature Map

| # | Feature | Module / Crate | Status |
|---|---------|---------------|--------|
| F01 | Email + Password Registration | `auth` | ⬜ |
| F02 | GitHub OAuth Login | `auth` | ⬜ |
| F03 | Google OAuth Login | `auth` | ⬜ |
| F04 | JWT + Refresh Token Auth | `auth` | ⬜ |
| F05 | Role-Based Access Control | `auth` | ⬜ |
| F06 | API Key Authentication | `auth` | ⬜ |
| F07 | Connect GitHub Account | `github` | ⬜ |
| F08 | Repository Listing from GitHub | `github` | ⬜ |
| F09 | Full Repository Sync | `github` | ⬜ |
| F10 | Incremental Repository Sync | `github` | ⬜ |
| F11 | Commit History Sync | `github` | ⬜ |
| F12 | Webhook Event Processing | `github` | ⬜ |
| F13 | Language Detection | `parser` | ⬜ |
| F14 | Framework Detection | `parser` | ⬜ |
| F15 | Database Detection | `parser` | ⬜ |
| F16 | Cloud Provider Detection | `parser` | ⬜ |
| F17 | CI/CD Detection | `parser` | ⬜ |
| F18 | Architecture Pattern Detection | `parser` | ⬜ |
| F19 | AI Resume Generation (JSON) | `ai` + `resume` | ⬜ |
| F20 | Resume PDF Export | `resume` | ⬜ |
| F21 | Resume DOCX Export | `resume` | ⬜ |
| F22 | ATS Resume Variant | `resume` | ⬜ |
| F23 | Resume Version History | `resume` | ⬜ |
| F24 | ATS Keyword Analysis | `ats` | ⬜ |
| F25 | ATS Score (0–100) | `ats` | ⬜ |
| F26 | ATS Improvement Suggestions | `ats` | ⬜ |
| F27 | Portfolio Data Generation | `portfolio` | ⬜ |
| F28 | Portfolio Website Publish | `portfolio` | ⬜ |
| F29 | Custom Domain for Portfolio | `portfolio` | ⬜ |
| F30 | Technology Usage Analytics | `analytics` | ⬜ |
| F31 | Contribution Score | `analytics` | ⬜ |
| F32 | Career Progress Dashboard | `analytics` | ⬜ |
| F33 | Hybrid Search (FTS + Vector) | `search` | ⬜ |
| F34 | Search Projects | `search` | ⬜ |
| F35 | Search Repositories | `search` | ⬜ |
| F36 | Search Skills | `search` | ⬜ |
| F37 | Career Goals Management | `career` | ⬜ |
| F38 | Career Timeline Events | `career` | ⬜ |
| F39 | AI Career Recommendations | `recommendation` | ⬜ |
| F40 | Job Application Tracker | `jobs` | ⬜ |
| F41 | Interview Practice (AI) | `interview` | ⬜ |
| F42 | Skill Learning Recommendations | `learning` | ⬜ |
| F43 | Email Notifications | `notification` | ⬜ |
| F44 | In-App Notifications | `notification` | ⬜ |
| F45 | Digest Email | `notification` | ⬜ |
| F46 | Background Job Workers (8) | `apps/worker` | ⬜ |
| F47 | Dead Letter Queue | `apps/worker` | ⬜ |
| F48 | Prometheus Metrics | `apps/api` | ⬜ |
| F49 | OpenTelemetry Tracing | `apps/api` | ⬜ |
| F50 | Health / Ready / Live endpoints | `apps/api` | ⬜ |

---

## Feature Status Key

| Symbol | Meaning |
|--------|---------|
| ⬜ | Not started |
| 🔄 | In progress |
| ✅ | Complete + tested |
| 🚫 | Blocked |

---

## MVP Features (Phase 1–4 Priority)

The following features form the MVP and must be completed before Phase 5+:

1. F01–F06 — Auth
2. F07–F12 — GitHub
3. F13–F18 — Parser
4. F19–F23 — Resume
5. F33–F36 — Search
6. F46–F50 — Infra
