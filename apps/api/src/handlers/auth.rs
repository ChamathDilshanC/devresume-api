use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use auth::{
    create_jwt, generate_refresh_token, hash_password, verify_password, AuthUser,
    GitHubOAuthClient, GoogleOAuthClient, OAuthCallbackQuery,
};
use common::models::User;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "DB error"})),
            )
        })?;

    if existing_user.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "Email already registered"})),
        ));
    }

    let password_hash = if let Some(pw) = &payload.password {
        Some(hash_password(pw).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Hash error"})),
            )
        })?)
    } else {
        None
    };

    let user_id = Uuid::new_v4();
    let now = Utc::now();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash, name, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'developer', $5, $6)
        RETURNING id, email, password_hash, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at
        "#)
        .bind(user_id)
        .bind(&payload.email)
        .bind(password_hash)
        .bind(&payload.name)
        .bind(now)
        .bind(now)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Insert error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Could not create user"})))
    })?;

    let access_token = create_jwt(user.id, &user.email, &state.config.jwt_secret).unwrap();
    let refresh_token = generate_refresh_token();

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    let user = sqlx::query_as::<_, User>(
        r#"SELECT id, email, password_hash, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at FROM users WHERE email = $1"#)
        .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))))?
    .ok_or((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))))?;

    if let Some(pw) = &payload.password {
        if let Some(hash) = &user.password_hash {
            if !verify_password(pw, hash).unwrap_or(false) {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid credentials"})),
                ));
            }
        } else {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid credentials"})),
            ));
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Password required"})),
        ));
    }

    let access_token = create_jwt(user.id, &user.email, &state.config.jwt_secret).unwrap();
    let refresh_token = generate_refresh_token();

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        },
    }))
}

pub async fn me(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<User>, (StatusCode, Json<Value>)> {
    let uid = Uuid::parse_str(&auth_user.id).unwrap();
    let user = sqlx::query_as::<_, User>(
        r#"SELECT id, email, password_hash, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at FROM users WHERE id = $1"#)
        .bind(uid)
    .fetch_one(&state.db)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))?;

    Ok(Json(user))
}

pub async fn logout() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "logged_out"})))
}

pub async fn refresh() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "refreshed"})))
}

pub async fn google_login(State(state): State<AppState>) -> Redirect {
    let client = GoogleOAuthClient::new(
        state.config.google_client_id.clone(),
        state.config.google_client_secret.clone(),
        state.config.google_redirect_uri.clone(),
    );
    let url = client.get_authorization_url("random_state_123");
    Redirect::temporary(&url)
}

pub async fn google_callback(
    Query(query): Query<OAuthCallbackQuery>,
    State(state): State<AppState>,
) -> Result<Redirect, (StatusCode, Json<Value>)> {
    let client = GoogleOAuthClient::new(
        state.config.google_client_id.clone(),
        state.config.google_client_secret.clone(),
        state.config.google_redirect_uri.clone(),
    );

    let token_resp = client.exchange_code(&query.code).await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Token exchange failed: {}", e)})),
        )
    })?;

    let profile = client
        .get_user_profile(&token_resp.access_token)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Profile fetch failed: {}", e)})),
            )
        })?;

    let existing_user = sqlx::query_as::<_, User>(
        r#"SELECT id, email, password_hash, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at FROM users WHERE email = $1"#)
        .bind(&profile.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))))?;

    let user = match existing_user {
        Some(u) => u,
        None => {
            let user_id = Uuid::new_v4();
            let now = Utc::now();
            sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (id, email, name, avatar_url, role, created_at, updated_at)
                VALUES ($1, $2, $3, $4, 'developer', $5, $6)
                RETURNING id, email, password_hash, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at
                "#)
                .bind(user_id)
                .bind(&profile.email)
                .bind(profile.name.unwrap_or_else(|| "Google User".to_string()))
                .bind(profile.picture)
                .bind(now)
                .bind(now)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Insert error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Could not create user"})))
            })?
        }
    };

    let access_token = create_jwt(user.id, &user.email, &state.config.jwt_secret).unwrap();
    let refresh_token = generate_refresh_token();

    let redirect_url = format!(
        "{}/auth/callback?access_token={}&refresh_token={}",
        state.config.web_url, access_token, refresh_token
    );
    Ok(Redirect::temporary(&redirect_url))
}

pub async fn github_login(State(state): State<AppState>) -> Redirect {
    let client = GitHubOAuthClient::new(
        state.config.github_client_id.clone(),
        state.config.github_client_secret.clone(),
        state.config.github_callback_url.clone(),
    );
    let url = client.get_authorization_url("random_state_456");
    Redirect::temporary(&url)
}

pub async fn github_callback(
    Query(query): Query<OAuthCallbackQuery>,
    State(state): State<AppState>,
) -> Result<Redirect, (StatusCode, Json<Value>)> {
    let client = GitHubOAuthClient::new(
        state.config.github_client_id.clone(),
        state.config.github_client_secret.clone(),
        state.config.github_callback_url.clone(),
    );

    let token_resp = client.exchange_code(&query.code).await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Token exchange failed: {}", e)})),
        )
    })?;

    let profile = client
        .get_user_profile(&token_resp.access_token)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Profile fetch failed: {}", e)})),
            )
        })?;

    let email = profile
        .email
        .unwrap_or_else(|| format!("{}@github.local", profile.login));

    let existing_user = sqlx::query_as::<_, User>(
        r#"SELECT id, email, password_hash, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at FROM users WHERE email = $1"#)
        .bind(&email)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))))?;

    let user = match existing_user {
        Some(u) => u,
        None => {
            let user_id = Uuid::new_v4();
            let now = Utc::now();
            let github_id_i64 = profile.id as i64;
            sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (id, email, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, 'developer', $5, $6, $7, $8, $9)
                RETURNING id, email, password_hash, name, avatar_url, role, bio, github_username, github_id, created_at, updated_at
                "#)
                .bind(user_id)
                .bind(email)
                .bind(profile.name.unwrap_or(profile.login.clone()))
                .bind(profile.avatar_url)
                .bind(profile.bio)
                .bind(profile.login)
                .bind(github_id_i64)
                .bind(now)
                .bind(now)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Insert error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Could not create user"})))
            })?
        }
    };

    let access_token = create_jwt(user.id, &user.email, &state.config.jwt_secret).unwrap();
    let refresh_token = generate_refresh_token();

    let redirect_url = format!(
        "{}/auth/callback?access_token={}&refresh_token={}",
        state.config.web_url, access_token, refresh_token
    );
    Ok(Redirect::temporary(&redirect_url))
}
