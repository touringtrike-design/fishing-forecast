use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{services::geocoding::detect_region, AppState};

/// Query parameters for region detection.
#[derive(Debug, Deserialize)]
pub struct RegionQuery {
    pub lat: f64,
    pub lon: f64,
}

/// Detect country for given coordinates.
pub async fn region_detect_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<RegionQuery>,
) -> impl IntoResponse {
    match detect_region(&state.http, query.lat, query.lon).await {
        Ok(region) => Json(region).into_response(),
        Err(err) => {
            let body = serde_json::json!({"error": err.to_string()});
            (axum::http::StatusCode::BAD_GATEWAY, Json(body)).into_response()
        }
    }
}
