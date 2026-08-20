---
title: DevResume API
emoji: 🚀
colorFrom: blue
colorTo: indigo
sdk: docker
app_port: 7860
---

# DevResume AI - Rust API Backend

> High-Performance Micro-Crate Rust Backend Engine powering **DevResume AI**.

---

## ⚡ Technology Stack

- **Language**: Rust 2021
- **Framework**: Axum
- **Async Runtime**: Tokio
- **Database**: PostgreSQL 16 + `pgvector`
- **ORM & Migrations**: SQLx
- **Cache**: Redis
- **Auth**: JWT & Bcrypt

---

## 🏗️ Architecture

```
devresume-api/
│
├── apps/
│   ├── api/          # Axum HTTP API Server (:8080)
│   ├── worker/       # Background Tokio Queue & Processing Engine
│   └── cli/          # DevResume Admin CLI
│
├── crates/
│   ├── common/       # Configuration, DB Pool, Error Types, Models
│   ├── auth/         # JWT Generation, Password Hashing & OAuth
│   ├── github/       # GitHub REST Client & Webhook Handler
│   ├── parser/       # Source Code & Dependency File Detector
│   ├── ai/           # AI Prompt Engine & Provider Abstraction
│   ├── resume/       # Resume JSON & Typst PDF Generator
│   ├── portfolio/    # Portfolio Builder Engine
│   ├── ats/          # ATS Keyword & Scoring Engine
│   ├── analytics/    # Developer Activity & Impact Score Calculator
│   ├── notification/ # Email & Alert Handlers
│   ├── search/       # Vector Search Client (pgvector)
│   └── storage/      # MinIO / S3 Document Client
│
├── migrations/       # SQLx PostgreSQL Migrations
└── Cargo.toml
```

---

## 👨‍💻 Maintainer

Author: **ChamathDilshanC** (`dilshancolonne123@gmail.com`)
