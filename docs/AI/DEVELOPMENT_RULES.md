# DevResume AI — AI Agent Development Rules

These rules govern how AI coding agents must behave when working on the DevResume AI backend.
Every rule is mandatory. No exceptions.

---

## 1. Identity & Attribution

- Author on all code, configs, and documentation: **`ChamathDilshanC <dilshancolonne123@gmail.com>`**
- Never add AI assistant attribution to any file.
- Never mention AI tools in commit messages, code comments, or documentation.

---

## 2. Architecture Preservation

```
NEVER:
├── Rewrite existing architecture
├── Rename crates
├── Rename repositories
├── Change folder structure
├── Remove or modify SQL migrations
├── Delete working documentation
├── Remove existing tests
├── Introduce duplicate modules
├── Hardcode secrets or credentials
└── Ignore compiler warnings
```

---

## 3. Pre-Implementation Checklist

Before writing any code for a feature, complete this checklist:

- [ ] Read `ARCHITECTURE.md`
- [ ] Read `docs/DATABASE.md`
- [ ] Read `docs/API_SPEC.md`
- [ ] Read `docs/DOMAIN_MODEL.md`
- [ ] Check if the feature already exists in any crate
- [ ] Check if any migration already covers the schema change
- [ ] Check if any endpoint already serves this functionality
- [ ] Confirm the module placement follows existing domain boundaries

---

## 4. Implementation Order

Always implement in this order for each module:

1. Database migration (if schema change required)
2. Domain models (`models.rs`)
3. Repository / data access layer
4. Domain service / business logic
5. Axum route handler
6. Request/Response DTOs with validation
7. Error types integration
8. Unit tests
9. Integration tests
10. Documentation update
11. OpenAPI annotation

---

## 5. Code Quality Gates (per module)

Every module must pass all before committing:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo check
cargo test
```

---

## 6. Git Commit Format

All commits must follow Conventional Commits:

```
feat(auth): implement github oauth flow
feat(ai): implement embeddings pipeline
feat(search): implement hybrid rrf search
fix(api): improve error response mapping
refactor(parser): optimize dependency detection
test(auth): add integration tests for jwt flow
docs(api): update openapi spec for v2 endpoints
chore(deps): update sqlx to 0.8
```

**No commit should break the build.**

---

## 7. Backward Compatibility

- Never remove existing public API endpoints.
- If an endpoint must change, version it under `/api/v2`.
- Never rename existing database columns — add new columns instead.
- Never remove enum variants — deprecate them with `#[deprecated]`.
- Never change existing error codes in API responses.

---

## 8. Security Rules

- Use `argon2` for password hashing. Never use `md5`, `sha1`, or `bcrypt` for new code.
- All tokens must be stored hashed, never plaintext.
- Never log sensitive data (passwords, tokens, API keys, PII).
- Validate all inputs at the API boundary — reject before reaching domain logic.
- Use prepared statements exclusively — no raw SQL string interpolation.
- CSRF tokens required for all state-changing browser requests.
- Rate limiting required on all authentication endpoints.

---

## 9. Error Handling

- All errors must be typed — no `.unwrap()` or `.expect()` in production code.
- Use `thiserror` for domain errors.
- Map all errors to appropriate HTTP status codes via `IntoResponse`.
- Include correlation IDs in all error responses.
- Log errors with `tracing::error!` including context.

---

## 10. Testing Requirements

- Minimum 80% unit test coverage per module.
- Every public function must have at least one test.
- Every Axum route must have an integration test.
- Every SQL repository method must have a database test.
- Tests must be deterministic — no random order dependencies.
- Use `tokio::test` for async tests.
- Use `sqlx::test` for database tests.
