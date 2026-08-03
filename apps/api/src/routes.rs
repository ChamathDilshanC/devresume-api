use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))

        // --- V1 Routes ---
        .route("/api/v1/auth/login", post(login_v1))
        .route("/api/v1/repositories", get(list_repositories_v1))
        .route("/api/v1/resumes/generate", post(generate_resume_v1))
        .route("/api/v1/ats/score", post(ats_score_v1))
        .route("/api/v1/analytics/overview", get(analytics_v1))

        // --- V2 Enterprise Routes ---
        .route("/api/v2/search/hybrid", post(hybrid_search_v2))
        .route("/api/v2/career/insights", get(career_insights_v2))
        .route("/api/v2/jobs/applications", get(job_applications_v2))
        .route("/api/v2/interview/practice", post(interview_practice_v2))
        .route("/api/v2/recommendations", get(recommendations_v2))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "devresume-api",
        "architecture": "enterprise-modular-monolith-9.8",
        "version": "2.0.0",
        "author": "ChamathDilshanC"
    }))
}

async fn login_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "status": "success",
        "token_type": "Bearer"
    }))
}

async fn list_repositories_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "repositories": []
    }))
}

async fn generate_resume_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "status": "success",
        "resume_id": uuid::Uuid::new_v4().to_string()
    }))
}

async fn ats_score_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "overall_score": 92
    }))
}

async fn analytics_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "impact_score": 510.0
    }))
}

async fn hybrid_search_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "engine": "hybrid-rrf-vector-keyword",
        "results": []
    }))
}

async fn career_insights_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "trajectory": "Senior Backend / Systems Engineer",
        "recommended_skills": ["Kubernetes", "Rust", "gRPC"]
    }))
}

async fn job_applications_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "applications": []
    }))
}

async fn interview_practice_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "questions": [
            {
                "question": "How does Tokio event loop handle async IO tasks?",
                "category": "Systems Engineering"
            }
        ]
    }))
}

async fn recommendations_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "recommendations": [
            {
                "title": "Add Microservice Architecture Project",
                "impact": "+25% Senior Rating"
            }
        ]
    }))
}
