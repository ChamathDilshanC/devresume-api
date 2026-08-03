# DevResume AI — Testing Guide

All tests must be written before a module is considered complete. This guide defines testing standards, patterns, and required coverage.

---

## Testing Levels

| Level | Location | Tool | Coverage Target |
|-------|----------|------|----------------|
| Unit Tests | `src/<module>.rs` `#[cfg(test)]` block | `cargo test` | 80%+ per module |
| Integration Tests | `tests/<module>_tests.rs` | `cargo test` | All public APIs |
| Database Tests | `tests/db/` | `sqlx::test` | All repository methods |
| API Tests | `tests/api/` | `axum-test` or `reqwest` | All HTTP routes |
| Worker Tests | `tests/workers/` | `tokio::test` | All worker handlers |

---

## Unit Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_round_trip() {
        let password = "secure_password_123!";
        let hash = hash_password(password).expect("hash failed");
        let is_valid = verify_password(password, &hash).expect("verify failed");
        assert!(is_valid, "Password verification should succeed");
    }

    #[test]
    fn test_invalid_password_rejected() {
        let hash = hash_password("correct_password").unwrap();
        let is_valid = verify_password("wrong_password", &hash).unwrap();
        assert!(!is_valid, "Wrong password should be rejected");
    }
}
```

---

## Async Unit Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_jwt_creation_and_verification() {
        let user_id = uuid::Uuid::new_v4();
        let email = "test@example.com";
        let secret = "test_secret_key";

        let token = create_jwt(user_id, email, secret).expect("JWT creation failed");
        let claims = verify_jwt(&token, secret).expect("JWT verification failed");

        assert_eq!(claims.email, email);
        assert_eq!(claims.sub, user_id.to_string());
    }
}
```

---

## Database Test Pattern (SQLx)

```rust
#[sqlx::test]
async fn test_user_repository_create_and_find(pool: PgPool) {
    let repo = UserRepository::new(&pool);

    let new_user = CreateUser {
        email: "test@devresume.ai".to_string(),
        name: "Test Developer".to_string(),
        password_hash: None,
    };

    let created = repo.create(new_user).await.expect("Create failed");
    assert_eq!(created.email, "test@devresume.ai");

    let found = repo
        .find_by_id(created.id)
        .await
        .expect("Find failed")
        .expect("User not found");

    assert_eq!(found.id, created.id);
}
```

---

## Axum API Test Pattern

```rust
#[tokio::test]
async fn test_health_endpoint_returns_200() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_with_invalid_credentials_returns_401() {
    let app = create_test_app().await;

    let body = serde_json::json!({
        "email": "nobody@example.com",
        "password": "wrong_password"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
```

---

## CI Test Command

Always run the full test suite before committing:

```bash
# Format check
cargo fmt --check

# Linting — no warnings allowed
cargo clippy -- -D warnings

# Build check
cargo check

# All tests
cargo test -- --test-threads=4

# Database integration tests (requires running PostgreSQL)
DATABASE_URL=postgres://... cargo test --test integration
```

---

## Coverage Targets

| Module | Unit Tests | Integration Tests | Min Coverage |
|--------|-----------|------------------|-------------|
| `auth` | ✅ Required | ✅ Required | 85% |
| `github` | ✅ Required | ✅ Required | 80% |
| `parser` | ✅ Required | ✅ Required | 80% |
| `ai` | ✅ Required | ⚠️ Mock provider OK | 75% |
| `resume` | ✅ Required | ✅ Required | 80% |
| `search` | ✅ Required | ✅ Required | 80% |
| `analytics` | ✅ Required | ✅ Required | 75% |
| `workers` | ✅ Required | ✅ Required | 75% |

---

## Test Data Rules

- Never use production credentials in tests.
- Use `dotenvy` to load `.env.test` for test runs.
- Clean up created test records after each test (use transactions or `sqlx::test` rollback).
- Never depend on test execution order.
- Mark slow tests with `#[ignore]` and a comment explaining why.

---

## Testing Anti-Patterns (Forbidden)

```
❌ Testing implementation details instead of behavior
❌ Using production database for tests
❌ Tests that pass only in specific order
❌ Hardcoded test credentials
❌ Mocking everything — integration tests must hit real database
❌ Ignoring test failures ("it usually works")
❌ Tests without assertions (empty test bodies)
```
