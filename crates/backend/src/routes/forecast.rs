use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{services::prediction::{build_forecast_ml, get_detailed_prediction, get_feature_importance}, AppState};

/// Query parameters for forecast requests.
#[derive(Debug, Deserialize)]
pub struct ForecastQuery {
    pub lat: f64,
    pub lon: f64,
    pub fish: Option<String>,
}

/// Get forecast for a location using ML model.
pub async fn forecast_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<ForecastQuery>,
) -> impl IntoResponse {
    match build_forecast_ml(&state.http, query.lat, query.lon, query.fish.as_deref()).await {
        Ok(result) => Json(result).into_response(),
        Err(err) => {
            let message = format!("{:#}", err);
            tracing::error!("Forecast error: {}", message);
            let body = serde_json::json!({"error": message});
            (axum::http::StatusCode::BAD_GATEWAY, Json(body)).into_response()
        }
    }
}

/// Get detailed ML prediction with factor breakdown.
pub async fn detailed_forecast_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<ForecastQuery>,
) -> impl IntoResponse {
    match get_detailed_prediction(&state.http, query.lat, query.lon).await {
        Ok(result) => Json(result).into_response(),
        Err(err) => {
            let message = format!("{:#}", err);
            tracing::error!("Detailed forecast error: {}", message);
            let body = serde_json::json!({"error": message});
            (axum::http::StatusCode::BAD_GATEWAY, Json(body)).into_response()
        }
    }
}

/// Get ML model feature importance scores.
pub async fn feature_importance_handler() -> impl IntoResponse {
    let importance = get_feature_importance().await;
    Json(importance).into_response()
}
