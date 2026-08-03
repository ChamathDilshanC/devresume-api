use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub bio: Option<String>,
    pub github_username: Option<String>,
    pub github_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Repository {
    pub id: Uuid,
    pub user_id: Uuid,
    pub github_repo_id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub html_url: String,
    pub description: Option<String>,
    pub default_branch: String,
    pub is_private: bool,
    pub language: Option<String>,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub is_synced: bool,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repository_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub architecture_pattern: Option<String>,
    pub deployment_status: Option<String>,
    pub live_url: Option<String>,
    pub repo_url: Option<String>,
    pub is_featured: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ResumeVersion {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub template_type: String,
    pub is_default: bool,
    pub content: serde_json::Value,
    pub pdf_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PortfolioPage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub custom_domain: Option<String>,
    pub theme: String,
    pub title: String,
    pub bio: Option<String>,
    pub is_published: bool,
    pub view_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AtsReport {
    pub id: Uuid,
    pub resume_version_id: Uuid,
    pub overall_score: i32,
    pub keyword_matches: serde_json::Value,
    pub formatting_score: i32,
    pub suggestions: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
