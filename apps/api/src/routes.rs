use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/repositories", get(list_repositories))
        .route("/api/v1/resumes/generate", post(generate_resume_handler))
        .route("/api/v1/ats/score", post(ats_score_handler))
        .route("/api/v1/analytics/overview", get(analytics_handler))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "devresume-api",
        "version": "0.1.0",
        "author": "ChamathDilshanC"
    }))
}

async fn login_handler() -> Json<Value> {
    Json(json!({
        "message": "Authentication endpoint initialized",
        "token_type": "Bearer"
    }))
}

async fn list_repositories() -> Json<Value> {
    Json(json!({
        "repositories": [],
        "synced_count": 0
    }))
}

async fn generate_resume_handler() -> Json<Value> {
    Json(json!({
        "status": "success",
        "resume_id": uuid::Uuid::new_v4().to_string(),
        "template": "modern"
    }))
}

async fn ats_score_handler() -> Json<Value> {
    Json(json!({
        "score": 92,
        "formatting_score": 95,
        "keywords": ["Rust", "PostgreSQL", "Axum"]
    }))
}

async fn analytics_handler() -> Json<Value> {
    Json(json!({
        "total_repositories": 12,
        "total_commits": 340,
        "impact_score": 510.0
    }))
}
