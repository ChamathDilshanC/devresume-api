# DevResume AI — Contributing Guide

> Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)

---

## Branch Strategy

| Branch | Purpose |
|--------|---------|
| `main` | Production-ready code. All CI must pass. |
| `develop` | Integration branch for features |
| `feat/<name>` | New feature development |
| `fix/<name>` | Bug fixes |
| `refactor/<name>` | Non-functional refactoring |
| `docs/<name>` | Documentation only |

---

## Development Workflow

```
1. Create feature branch from develop
   git checkout -b feat/auth-github-oauth

2. Implement the feature following MODULE_CHECKLIST.md

3. After each sub-feature:
   cargo fmt
   cargo clippy -- -D warnings
   cargo check
   cargo test

4. Commit with conventional commit format
   git commit -m "feat(auth): implement github oauth callback"

5. Push branch and open pull request against develop

6. All CI checks must pass before merge
```

---

## CI Checks (Required to Pass)

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo check
cargo test
```

---

## Authorship Rule

**All commits must have the correct author set.**

```bash
git config user.name "ChamathDilshanC"
git config user.email "dilshancolonne123@gmail.com"
```

Verify before pushing:

```bash
git log --oneline -5 --format="%an <%ae>"
```

---

## Pull Request Requirements

- [ ] Title follows Conventional Commits format
- [ ] All CI checks pass
- [ ] New feature has tests
- [ ] Documentation updated if API surface changed
- [ ] No debug code, `println!`, or temporary comments
- [ ] `cargo clippy -- -D warnings` passes
- [ ] No merge conflicts with `develop`
- [ ] Author is `ChamathDilshanC <dilshancolonne123@gmail.com>`

---

## What NOT to Do

```
❌ Merge to main directly (only via PR)
❌ Push broken code to develop
❌ Force-push to main or develop
❌ Commit with wrong author
❌ Add AI assistant attribution to any file
❌ Commit secrets or .env files
```
