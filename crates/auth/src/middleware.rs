use crate::jwt::{verify_jwt, Claims};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

pub struct AuthError(pub StatusCode, pub String);

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.1,
            "status": self.0.as_u16(),
        }));
        (self.0, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|val| val.to_str().ok())
            .ok_or_else(|| {
                AuthError(
                    StatusCode::UNAUTHORIZED,
                    "Missing Authorization header".to_string(),
                )
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            AuthError(
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization scheme".to_string(),
            )
        })?;

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "devresume_jwt_secret_key_32_chars_min".to_string());

        let claims: Claims = verify_jwt(token, &jwt_secret).map_err(|_| {
            AuthError(
                StatusCode::UNAUTHORIZED,
                "Invalid or expired JWT token".to_string(),
            )
        })?;

        Ok(AuthUser {
            id: claims.sub,
            email: claims.email,
        })
    }
}
