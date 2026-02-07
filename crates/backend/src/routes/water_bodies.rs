use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{db::queries, AppState};

/// Query parameters for nearby water bodies.
#[derive(Debug, Deserialize)]
pub struct WaterBodiesQuery {
    pub lat: f64,
    pub lon: f64,
    pub radius_km: f64,
}

/// Get nearby water bodies.
pub async fn water_bodies_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<WaterBodiesQuery>,
) -> impl IntoResponse {
    match queries::get_nearby_water_bodies(&state.db, query.lat, query.lon, query.radius_km).await {
        Ok(rows) => Json::<Vec<_>>(rows).into_response(),
        Err(err) => {
            let body: serde_json::Value = serde_json::json!({"error": err.to_string()});
            (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
    }
}
