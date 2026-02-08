use axum::{extract::State, response::IntoResponse, Json};

use crate::{services::auth::{self, LoginRequest, RegisterRequest}, AppState};

/// Register new user
/// POST /api/v1/auth/register
pub async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    match auth::register(&state, request).await {
        Ok(response) => (axum::http::StatusCode::CREATED, Json(response)).into_response(),
        Err(err) => {
            let body = serde_json::json!({ "error": err.to_string() });
            (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
    }
}

/// Login user
/// POST /api/v1/auth/login
pub async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    match auth::login(&state, request).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            let body = serde_json::json!({ "error": err.to_string() });
            (axum::http::StatusCode::UNAUTHORIZED, Json(body)).into_response()
        }
    }
}

/// Get current user profile
/// GET /api/v1/auth/me
pub async fn me_handler(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // Extract token from Authorization header
    let token = match extract_token(&headers) {
        Some(t) => t,
        None => {
            let body = serde_json::json!({ "error": "Missing authorization header" });
            return (axum::http::StatusCode::UNAUTHORIZED, Json(body)).into_response();
        }
    };
    
    // Validate token
    let claims = match auth::validate_token(&token) {
        Ok(c) => c,
        Err(err) => {
            let body = serde_json::json!({ "error": err.to_string() });
            return (axum::http::StatusCode::UNAUTHORIZED, Json(body)).into_response();
        }
    };
    
    // Get user
    match auth::get_user_by_id(&state, &claims.sub).await {
        Ok(Some(user)) => (axum::http::StatusCode::OK, Json(user)).into_response(),
        Ok(None) => {
            let body = serde_json::json!({ "error": "User not found" });
            (axum::http::StatusCode::NOT_FOUND, Json(body)).into_response()
        }
        Err(err) => {
            let body = serde_json::json!({ "error": err.to_string() });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        }
    }
}

/// Extract JWT token from Authorization header
fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()
        .filter(|s| s.starts_with("Bearer "))
        .map(|s| s.trim_start_matches("Bearer ").to_string())
}
