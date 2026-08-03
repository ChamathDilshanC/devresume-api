# DevResume AI — Definition of DONE

This document defines the exact acceptance criteria that must be satisfied for any piece of work to be considered complete.

**No exceptions. No shortcuts.**

---

## Level 1: A Function is DONE when

- [ ] It compiles without errors
- [ ] It has no clippy warnings
- [ ] It uses proper error handling (no `.unwrap()` in production paths)
- [ ] It has at least one unit test
- [ ] It is documented with a doc comment (`///`) if public
- [ ] It logs appropriately with `tracing::{info, error, warn, debug}`
- [ ] It is async if it performs I/O

---

## Level 2: A Database Repository Method is DONE when

- [ ] Uses `sqlx::query_as!` or `sqlx::query!` macros (not raw strings)
- [ ] Returns `Result<_, sqlx::Error>` or domain error wrapping it
- [ ] Handles `NOT FOUND` case explicitly (returns `Option<T>`)
- [ ] Includes soft-delete awareness (`WHERE deleted_at IS NULL`)
- [ ] Has a database integration test using `#[sqlx::test]`
- [ ] Uses correct index on filtered columns
- [ ] Pagination uses `LIMIT` + `OFFSET` or cursor-based

---

## Level 3: An API Endpoint is DONE when

- [ ] Route is registered in the versioned router (`/api/v1` or `/api/v2`)
- [ ] Request body is validated with `garde` or `validator`
- [ ] Returns correct HTTP status codes (200, 201, 400, 401, 403, 404, 422, 500)
- [ ] Returns consistent JSON error response shape:
  ```json
  {
    "error": "Descriptive error message",
    "status": 422,
    "request_id": "uuid-here"
  }
  ```
- [ ] Authentication/Authorization is enforced (middleware or extractor)
- [ ] Integration test exists that asserts status code + response body
- [ ] OpenAPI annotation is added

---

## Level 4: A Domain Module (Crate) is DONE when

- [ ] All functions in `service.rs` have unit tests
- [ ] All functions in `repository.rs` have database tests
- [ ] All Axum handlers in `handlers.rs` have integration tests
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo check` passes
- [ ] `cargo test` passes (including the new tests)
- [ ] Domain errors in `errors.rs` are typed and map to HTTP status codes
- [ ] Structured logging with correlation IDs
- [ ] Input validation at the DTO boundary
- [ ] Documentation updated in `docs/`
- [ ] Migration exists in `migrations/` if schema changed
- [ ] Committed with author `ChamathDilshanC <dilshancolonne123@gmail.com>`

---

## Level 5: A Background Worker is DONE when

- [ ] Worker task spawned with `tokio::spawn`
- [ ] Pulls jobs from `job_queue` table using advisory locks
- [ ] Updates job status (`processing` → `completed` / `failed`)
- [ ] Retry logic implemented (max retries from `jobs.max_retries`)
- [ ] Failed-beyond-max-retries jobs moved to `dead_letter_jobs`
- [ ] Structured logging per job execution (job id, duration, status)
- [ ] Worker has a test that verifies it processes a seeded job correctly
- [ ] Worker gracefully shuts down on SIGTERM

---

## Level 6: The Entire Backend is DONE when

```
✓ cargo fmt --check        → passes
✓ cargo clippy -D warnings → 0 warnings
✓ cargo check              → 0 errors
✓ cargo test               → all pass (coverage ≥ 80%)
✓ sqlx migrate run         → all migrations applied
✓ docker compose up        → all services healthy
✓ GET /health              → 200 OK
✓ GET /ready               → 200 OK
✓ GET /live                → 200 OK
✓ GET /metrics             → Prometheus metrics
✓ GitHub OAuth             → works end-to-end
✓ Repository sync          → works end-to-end
✓ AI provider              → at least one provider works
✓ Resume generate          → PDF produced
✓ Portfolio                → JSON/published data correct
✓ ATS score                → returns score 0–100
✓ Hybrid search            → returns ranked results
✓ Background workers       → all 8 workers process jobs
✓ OpenAPI spec             → generated and accessible
✓ No runtime panics        → confirmed via load test
✓ No hardcoded secrets     → env vars used everywhere
✓ No TODO/FIXME/HACK       → grep returns empty
```

---

## Quick Reference

```
A function is DONE when: tested + typed errors + documented
A repo method is DONE when: sqlx + tested + soft-delete aware
An endpoint is DONE when: versioned + validated + tested + OpenAPI annotated
A module is DONE when: all clippy/fmt/check/test pass + docs updated
A worker is DONE when: processes + retries + moves to DLQ + graceful shutdown
The backend is DONE when: all of the above + Docker + CI all green
```
