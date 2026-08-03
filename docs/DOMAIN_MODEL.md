# DevResume AI — Domain Model

> Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)

This document describes the core domain entities, their relationships, and value objects used in the DevResume AI system.

---

## Core Aggregates

### User Aggregate (Root)
- `User` — identity, email, role, GitHub username
- `Account` — OAuth provider linkage (GitHub, Google)
- `Session` — active session with JWT + refresh token
- **Rules**: One user can have multiple OAuth accounts. Sessions expire.

### Repository Aggregate
- `Repository` — GitHub repo metadata, sync state
- `RepositoryEvent` — webhook event payloads
- `Commit` — commit diff stats, impact score
- **Rules**: Repository belongs to one User. Sync is incremental.

### Project Aggregate
- `Project` — auto-detected from repository analysis
- `ProjectVersion` — changelog entries per release
- `ProjectTechnology` — junction with detected tech stack
- **Rules**: Projects are derived from repositories, not manually created.

### Resume Aggregate
- `ResumeVersion` — generated JSON, PDF artifact path, template used
- `AtsReport` — ATS keyword match results for a resume version
- **Rules**: User can have multiple resume versions. One marked as default.

### Career Aggregate
- `CareerGoal` — target role, salary, timeframe
- `CareerTimeline` — milestone events (project shipped, skill gained)
- `Recommendation` — AI-generated personalized career advice
- **Rules**: Timeline is append-only. Goals have a status lifecycle.

---

## Value Objects

| Value Object | Description |
|-------------|-------------|
| `Email` | Validated email address — unique per user |
| `Role` | Enum: `admin` / `developer` / `recruiter` |
| `JobStatus` | Enum: `pending` / `processing` / `completed` / `failed` |
| `ExportFormat` | Enum: `pdf` / `docx` / `html` / `markdown` / `png` / `zip` |
| `TechnologyCategory` | Enum: `language` / `framework` / `database` / `cloud` / `devops` / `tool` |

---

## Entity Relationships

```
User
 ├── Account (1:n) — OAuth providers
 ├── Session (1:n) — Active sessions
 ├── Repository (1:n) — Tracked repos
 │    ├── Commit (1:n) — Commit history
 │    ├── RepositoryEmbedding (1:n) — Vector chunks
 │    └── Project (1:n) — Derived projects
 │         └── ProjectTechnology (n:m via junction)
 ├── ResumeVersion (1:n) — Resume history
 │    ├── AtsReport (1:1) — ATS scoring
 │    └── ResumeEmbedding (1:n) — Vector chunks
 ├── PortfolioPage (1:1) — Public portfolio
 ├── CareerGoal (1:n) — Growth targets
 ├── Notification (1:n) — Alerts
 ├── JobApplication (1:n) — Job tracker
 └── InterviewSession (1:n) — Mock interviews
```

---

## Lifecycle State Machines

### Job Queue Status

```
pending → processing → completed
                    ↘ failed → (retry) → dead_letter
```

### Resume Generation

```
trigger → ai_job(pending) → ai_job(processing) → resume_version(created) → artifact(pdf/docx)
```

### Repository Sync

```
connected → sync_queued → syncing → synced
                                 ↘ sync_failed
```
