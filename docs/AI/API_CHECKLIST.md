# DevResume AI — API Implementation Checklist

Track every REST API endpoint. Check off when implemented, tested, and documented.

---

## Versioning Policy

- `/api/v1` — Stable, backward-compatible endpoints
- `/api/v2` — Enterprise endpoints, may require newer clients
- All routes must be explicitly versioned — no bare `/api/*`

---

## Authentication — `/api/v1/auth`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| POST | `/api/v1/auth/register` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/auth/login` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/auth/refresh` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/auth/logout` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/auth/forgot-password` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/auth/reset-password` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/auth/verify-email` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/auth/github` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/auth/github/callback` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/auth/google` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/auth/google/callback` | ⬜ | ⬜ | ⬜ |

---

## Users — `/api/v1/users`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v1/users/me` | ⬜ | ⬜ | ⬜ |
| PUT  | `/api/v1/users/me` | ⬜ | ⬜ | ⬜ |
| DELETE | `/api/v1/users/me` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/users/me/api-keys` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/users/me/api-keys` | ⬜ | ⬜ | ⬜ |
| DELETE | `/api/v1/users/me/api-keys/:id` | ⬜ | ⬜ | ⬜ |

---

## Repositories — `/api/v1/repositories`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v1/repositories` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/repositories/sync` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/repositories/:id` | ⬜ | ⬜ | ⬜ |
| DELETE | `/api/v1/repositories/:id` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/repositories/:id/commits` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/repositories/:id/technologies` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/repositories/:id/stats` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/webhooks/github` | ⬜ | ⬜ | ⬜ |

---

## Projects — `/api/v1/projects`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v1/projects` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/projects` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/projects/:id` | ⬜ | ⬜ | ⬜ |
| PUT  | `/api/v1/projects/:id` | ⬜ | ⬜ | ⬜ |
| DELETE | `/api/v1/projects/:id` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/projects/:id/feature` | ⬜ | ⬜ | ⬜ |

---

## Resume — `/api/v1/resumes`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v1/resumes` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/resumes/generate` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/resumes/:id` | ⬜ | ⬜ | ⬜ |
| DELETE | `/api/v1/resumes/:id` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/resumes/:id/download?format=pdf` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/resumes/:id/download?format=docx` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/resumes/:id/set-default` | ⬜ | ⬜ | ⬜ |

---

## Portfolio — `/api/v1/portfolio`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v1/portfolio` | ⬜ | ⬜ | ⬜ |
| PUT  | `/api/v1/portfolio` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/portfolio/publish` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/portfolio/unpublish` | ⬜ | ⬜ | ⬜ |

---

## ATS — `/api/v1/ats`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| POST | `/api/v1/ats/score` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/ats/reports` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/ats/reports/:id` | ⬜ | ⬜ | ⬜ |

---

## Analytics — `/api/v1/analytics`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v1/analytics/overview` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v1/analytics/technologies` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v2/analytics/career` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v2/analytics/contributions` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v2/analytics/ranking` | ⬜ | ⬜ | ⬜ |

---

## Search — `/api/v2/search`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| POST | `/api/v2/search` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v2/search/vector` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v2/search/hybrid` | ⬜ | ⬜ | ⬜ |

---

## Notifications — `/api/v1/notifications`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v1/notifications` | ⬜ | ⬜ | ⬜ |
| PUT  | `/api/v1/notifications/:id/read` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v1/notifications/mark-all-read` | ⬜ | ⬜ | ⬜ |

---

## Career — `/api/v2/career`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v2/career/goals` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v2/career/goals` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v2/career/timeline` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v2/career/insights` | ⬜ | ⬜ | ⬜ |

---

## Jobs — `/api/v2/jobs`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v2/jobs/applications` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v2/jobs/applications` | ⬜ | ⬜ | ⬜ |
| PUT  | `/api/v2/jobs/applications/:id` | ⬜ | ⬜ | ⬜ |
| DELETE | `/api/v2/jobs/applications/:id` | ⬜ | ⬜ | ⬜ |

---

## Interview — `/api/v2/interview`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| POST | `/api/v2/interview/practice` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v2/interview/sessions` | ⬜ | ⬜ | ⬜ |
| GET  | `/api/v2/interview/sessions/:id` | ⬜ | ⬜ | ⬜ |

---

## Recommendations — `/api/v2/recommendations`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/api/v2/recommendations` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v2/recommendations/generate` | ⬜ | ⬜ | ⬜ |

---

## AI — `/api/v2/ai`

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| POST | `/api/v2/ai/generate` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v2/ai/embed` | ⬜ | ⬜ | ⬜ |
| POST | `/api/v2/ai/chat` | ⬜ | ⬜ | ⬜ |

---

## Health & Observability

| Method | Endpoint | Status | Tests | OpenAPI |
|--------|----------|--------|-------|---------|
| GET  | `/health` | ⬜ | ⬜ | ⬜ |
| GET  | `/ready` | ⬜ | ⬜ | ⬜ |
| GET  | `/live` | ⬜ | ⬜ | ⬜ |
| GET  | `/metrics` | ⬜ | ⬜ | ⬜ |
