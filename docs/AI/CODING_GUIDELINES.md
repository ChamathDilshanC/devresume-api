# DevResume AI — Coding Guidelines for AI Agents

These guidelines define idiomatic Rust coding standards for the DevResume AI backend. Follow them precisely in every file you produce.

---

## 1. Rust Edition & Toolchain

- **Edition**: Rust 2021 (2024 migration planned for next major version)
- **Toolchain**: Stable — no nightly features
- **Minimum Rust version**: 1.78.0

---

## 2. Project Structure

Every domain crate must follow this module layout:

```
crates/<domain>/
├── Cargo.toml
└── src/
    ├── lib.rs          # Public re-exports only
    ├── models.rs       # Domain entities and value objects
    ├── repository.rs   # Database access (trait + impl)
    ├── service.rs      # Business logic
    ├── handlers.rs     # Axum route handlers
    ├── dto.rs          # Request & Response types (validated)
    ├── errors.rs       # Domain-specific error enum
    └── tests/
        ├── unit.rs
        └── integration.rs
```

---

## 3. Naming Conventions

| Type | Convention | Example |
|------|-----------|---------|
| Types / Structs | `PascalCase` | `UserRepository` |
| Traits | `PascalCase` | `AIProvider` |
| Functions / Methods | `snake_case` | `find_by_email` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_TOKEN_COUNT` |
| Modules | `snake_case` | `resume_builder` |
| Enums | `PascalCase` | `JobStatus` |
| Enum variants | `PascalCase` | `JobStatus::Pending` |
| Axum extractors | `PascalCase` | `AuthUser` |
| DTOs | `<Action><Domain>Request/Response` | `CreateResumeRequest` |

---

## 4. Error Handling

```rust
// CORRECT — typed domain error
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Token expired")]
    TokenExpired,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

// CORRECT — never use unwrap/expect in production
let user = repo.find_by_id(id).await.map_err(AuthError::Database)?;

// WRONG — forbidden
let user = repo.find_by_id(id).await.unwrap();
```

---

## 5. Async Style

```rust
// CORRECT — async/await everywhere
pub async fn create_user(
    State(db): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = UserRepository::new(&db).create(req.into()).await?;
    Ok(Json(user.into()))
}

// CORRECT — use ? for early return
let repo = UserRepository::new(pool);
let user = repo.find_by_email(&email).await?;
```

---

## 6. Database Patterns

```rust
// CORRECT — typed SQLx queries
let user: Option<User> = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL",
    id
)
.fetch_optional(pool)
.await?;

// CORRECT — transactions for multi-step writes
let mut tx = pool.begin().await?;
sqlx::query!("INSERT INTO users ...", ...).execute(&mut *tx).await?;
sqlx::query!("INSERT INTO accounts ...", ...).execute(&mut *tx).await?;
tx.commit().await?;
```

---

## 7. Validation

```rust
// CORRECT — validate at the boundary using garde or validator
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}
```

---

## 8. Logging & Tracing

```rust
// CORRECT — structured tracing with spans
#[tracing::instrument(skip(pool, req), fields(user_email = %req.email))]
pub async fn login(pool: &PgPool, req: LoginRequest) -> Result<AuthToken, AuthError> {
    tracing::info!("Login attempt initiated");
    // ...
    tracing::info!(user_id = %user.id, "Login successful");
    Ok(token)
}

// WRONG — println!, eprintln!, dbg! in production
println!("login: {:?}", req);
```

---

## 9. Configuration

```rust
// CORRECT — environment-based configuration only
pub struct Config {
    pub database_url: String,     // DATABASE_URL
    pub redis_url: String,        // REDIS_URL
    pub jwt_secret: String,       // JWT_SECRET
    pub openai_api_key: String,   // OPENAI_API_KEY
}

// WRONG — hardcoded secrets
let jwt_secret = "super_secret_123";
```

---

## 10. Testing Standards

```rust
// CORRECT — unit test in same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let hash = hash_password("secure_password").unwrap();
        assert!(verify_password("secure_password", &hash).unwrap());
    }
}

// CORRECT — async integration test
#[tokio::test]
async fn test_create_user_returns_201() {
    let app = test_app().await;
    let response = app.post("/api/v1/users").json(&payload).send().await;
    assert_eq!(response.status(), StatusCode::CREATED);
}
```

---

## 11. Forbidden Patterns

```
❌ .unwrap() in production code
❌ .expect("...") in production code
❌ println! or eprintln! — use tracing
❌ Hardcoded secrets or credentials
❌ Raw SQL string formatting (SQL injection risk)
❌ Blocking operations in async context
❌ std::thread::sleep — use tokio::time::sleep
❌ Clippy-disabled code without justification comment
❌ Dead code (unused imports, dead functions)
❌ Duplicate logic copied from another module
```
