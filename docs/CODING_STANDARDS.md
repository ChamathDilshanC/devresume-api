# DevResume AI — Coding Standards

> Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)

These standards apply to every line of Rust code in this repository. Adherence is mandatory.

---

## Toolchain

```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
```

---

## Formatting

```bash
# Always format before committing
cargo fmt

# Format check in CI
cargo fmt --check
```

No custom rustfmt configuration overrides are permitted unless explicitly approved.

---

## Linting

```bash
# All clippy warnings are errors
cargo clippy -- -D warnings
```

Allowed suppression (must include justification comment):

```rust
// Safety: The raw pointer is guaranteed to be valid because ...
#[allow(clippy::arc_with_non_send_sync)]
```

Never suppress:

```
#[allow(unused)]
#[allow(dead_code)]
#[allow(clippy::unwrap_used)]
```

---

## Error Handling

1. Use `thiserror` for domain error enums.
2. Use `anyhow::Result` only at the application boundary (main.rs, CLI entrypoints).
3. Never use `.unwrap()` or `.expect()` in production code paths.
4. Map all errors to HTTP responses via `impl IntoResponse for AppError`.

---

## Logging

1. Use `tracing` — not `println!`, `eprintln!`, or `log!`.
2. Use `#[tracing::instrument]` on all service and repository methods.
3. Skip sensitive fields: `#[tracing::instrument(skip(password, token))]`.
4. Use structured fields: `tracing::info!(user_id = %id, "action description")`.

---

## Configuration

1. All settings loaded from environment variables.
2. Configuration struct validated at startup — fail fast on missing required vars.
3. `.env.example` in version control. Never commit `.env` with real values.

---

## Module Organization

```rust
// lib.rs — Public re-exports only
pub use handlers::router;
pub use models::User;
pub use errors::AuthError;

// No business logic in lib.rs
```

---

## Commit Message Format

All commits must follow Conventional Commits:

```
feat(<scope>): <short description>
fix(<scope>): <short description>
refactor(<scope>): <short description>
test(<scope>): <short description>
docs(<scope>): <short description>
chore(<scope>): <short description>
```

Scopes: `auth`, `github`, `parser`, `ai`, `resume`, `search`, `api`, `worker`, `db`, `deps`, `config`

---

## Dependency Rules

1. No duplicate crates for the same purpose.
2. Prefer workspace-level dependency declarations over per-crate.
3. Pin major versions only — allow minor/patch updates.
4. Run `cargo audit` before every release.
5. New external dependencies require brief justification in the PR.
