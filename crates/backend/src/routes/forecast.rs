use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{services::prediction::build_forecast, AppState};

/// Query parameters for forecast requests.
#[derive(Debug, Deserialize)]
pub struct ForecastQuery {
    pub lat: f64,
    pub lon: f64,
    pub fish: Option<String>,
}

/// Get forecast for a location.
pub async fn forecast_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<ForecastQuery>,
) -> impl IntoResponse {
    match build_forecast(&state.http, query.lat, query.lon, query.fish.as_deref()).await {
        Ok(result) => Json(result).into_response(),
        Err(err) => {
            let body = serde_json::json!({"error": err.to_string()});
            (axum::http::StatusCode::BAD_GATEWAY, Json(body)).into_response()
        }
    }
}
