# DevResume AI — Security Policy

> Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)

---

## Authentication Security

| Control | Implementation |
|---------|---------------|
| Password hashing | Argon2id with salt |
| JWT signing | HMAC-SHA256 with rotating secret |
| Access token expiry | 15 minutes |
| Refresh token expiry | 7 days, single-use rotation |
| OAuth state parameter | CSRF validation on OAuth callback |
| Secure cookies | `HttpOnly`, `Secure`, `SameSite=Strict` |
| Rate limiting | 10 req/min on auth endpoints |
| Brute-force protection | Account lockout after 5 failed attempts |

---

## API Security

| Control | Implementation |
|---------|---------------|
| HTTPS only | TLS termination at load balancer |
| CORS policy | Allowlist-based origin validation |
| Security headers | `Strict-Transport-Security`, `X-Content-Type-Options`, `X-Frame-Options` |
| Input validation | Validated at DTO boundary before domain logic |
| SQL injection | SQLx prepared statements only — no string formatting |
| Path traversal | File paths sanitized in storage layer |
| Request IDs | UUID injected per request for tracing |

---

## Secrets Management

```
NEVER:
- Hardcode secrets in source code
- Log secrets, tokens, or passwords
- Commit .env files
- Store secrets in environment variables in plain Docker images

ALWAYS:
- Load secrets from environment via config module
- Use .env.example (no real values) in version control
- Use Kubernetes Secrets or similar in production
- Rotate secrets regularly
```

---

## GitHub Webhook Security

```rust
// All webhook payloads must pass HMAC-SHA256 verification
// X-Hub-Signature-256: sha256=<hmac>
pub fn verify_github_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
    // Constant-time comparison to prevent timing attacks
}
```

---

## Data Privacy

- PII (email, name) must not appear in logs
- `tracing::instrument` must use `skip(password, token)` fields
- User data deletion must cascade to all tables (GDPR)
- Soft delete is NOT sufficient for GDPR requests — hard delete required on request

---

## Dependency Security

```bash
# Run before every release
cargo audit

# Keep dependencies up to date
cargo outdated
```

---

## Incident Response

For any suspected security issue:
1. Do not merge to `main`
2. Report privately via email: `dilshancolonne123@gmail.com`
3. Include affected version, reproduction steps, impact
