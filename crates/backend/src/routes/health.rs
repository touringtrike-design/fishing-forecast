use axum::Json;
use serde_json::json;

/// Health check endpoint.
pub async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}
