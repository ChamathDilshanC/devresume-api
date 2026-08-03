# ADR 0006 — Background Workers via PostgreSQL Job Queue

- **Status**: Accepted
- **Date**: 2026-01-01
- **Author**: ChamathDilshanC

---

## Context

DevResume AI requires background processing for: GitHub sync, embedding generation, resume compilation, PDF rendering, email delivery, and periodic cleanup. These tasks are slow (seconds to minutes), must not block HTTP responses, must be retried on failure, and must survive server restarts.

Options evaluated:
- **PostgreSQL job queue** (custom `job_queue` table)
- **Redis + Sidekiq-style queue** (via `bb8-redis` + custom worker)
- **RabbitMQ / AMQP** (dedicated message broker)
- **Apache Kafka** (event streaming)
- **Background tokio tasks** (in-process, no persistence)

---

## Decision

**Use a PostgreSQL-backed job queue with `SELECT ... FOR UPDATE SKIP LOCKED` for distributed worker coordination.**

---

## Schema

```sql
CREATE TABLE job_queue (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_type    TEXT NOT NULL,
    payload     JSONB NOT NULL DEFAULT '{}',
    status      job_status NOT NULL DEFAULT 'pending',
    priority    INT NOT NULL DEFAULT 0,
    attempts    INT NOT NULL DEFAULT 0,
    max_retries INT NOT NULL DEFAULT 3,
    scheduled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at  TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    failed_at   TIMESTAMPTZ,
    error       TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

## Worker Coordination

```sql
-- Claim next available job (advisory-lock-safe)
SELECT * FROM job_queue
WHERE status = 'pending'
  AND scheduled_at <= NOW()
ORDER BY priority DESC, scheduled_at ASC
LIMIT 1
FOR UPDATE SKIP LOCKED;
```

This guarantees exactly-once processing across multiple worker instances without Redis.

---

## Rationale

| Criterion | PG Job Queue | Redis Queue | RabbitMQ |
|-----------|-------------|-------------|---------|
| Infrastructure | ✅ Reuses PG | ⚠️ New service | ❌ New service |
| ACID durability | ✅ Full | ❌ Optional | ⚠️ Partial |
| Exactly-once | ✅ SKIP LOCKED | ⚠️ At-least-once | ⚠️ Complex |
| Dead letter queue | ✅ Same table | ⚠️ Extra config | ✅ Built-in |
| SQL JOIN with domain | ✅ Yes | ❌ No | ❌ No |
| Operational cost | ✅ Zero | ⚠️ Redis cost | ❌ High |
| Retry logic | ✅ SQL update | ✅ Built-in | ✅ Built-in |

---

## Workers Implemented

| Worker | Job Types |
|--------|----------|
| `sync_worker` | `github_sync`, `incremental_sync` |
| `github_worker` | `webhook_push`, `webhook_pr` |
| `resume_worker` | `generate_pdf`, `generate_docx` |
| `portfolio_worker` | `render_portfolio` |
| `embedding_worker` | `embed_repository`, `embed_project`, `embed_resume` |
| `notification_worker` | `send_email`, `send_inapp` |
| `cleanup_worker` | `expire_sessions`, `purge_jobs` |
| `scheduler_worker` | `daily_digest`, `weekly_sync` |

---

## Consequences

- **Do not introduce Redis, RabbitMQ, or Kafka** for job queuing — all jobs go through `job_queue`.
- Failed jobs exceeding `max_retries` move to `dead_letter_jobs` table.
- Workers use exponential backoff: `2^attempt` seconds delay.
- Each worker is a `tokio::spawn` loop in `apps/worker/`.
- Workers handle `SIGTERM` for graceful shutdown — finish current job, then exit.
