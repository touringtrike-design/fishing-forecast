use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{db::queries, models::NewCatchRecord, AppState};

/// Query parameters for fetching user catches.
#[derive(Debug, Deserialize)]
pub struct CatchesQuery {
    pub user_id: Uuid,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for nearby catches.
#[derive(Debug, Deserialize)]
pub struct NearbyCatchesQuery {
    pub lat: f64,
    pub lon: f64,
    pub radius_km: f64,
}

/// Save a new catch record.
pub async fn save_catch_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(payload): Json<NewCatchRecord>,
) -> impl IntoResponse {
    match queries::insert_catch(&state.db, &payload).await {
        Ok(saved) => {
            use crate::models::CatchRecordDb;
            Json::<CatchRecordDb>(saved).into_response()
        }
        Err(err) => {
            let error_msg = err.to_string();
            let body: serde_json::Value = serde_json::json!({"error": error_msg});
            (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
    }
}

/// Get catches for a user.
pub async fn get_catches_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<CatchesQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50).clamp(1, 200);
    let offset = query.offset.unwrap_or(0).max(0);

    match queries::get_catches_by_user(&state.db, query.user_id, limit, offset).await {
        Ok(rows) => {
            use crate::models::CatchRecordDb;
            Json::<Vec<CatchRecordDb>>(rows).into_response()
        }
        Err(err) => {
            let error_msg = err.to_string();
            let body: serde_json::Value = serde_json::json!({"error": error_msg});
            (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
    }
}

/// Get nearby public catches.
pub async fn get_nearby_catches_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<NearbyCatchesQuery>,
) -> impl IntoResponse {
    match queries::get_nearby_catches(&state.db, query.lat, query.lon, query.radius_km).await {
        Ok(rows) => {
            use crate::models::CatchRecordDb;
            Json::<Vec<CatchRecordDb>>(rows).into_response()
        }
        Err(err) => {
            let error_msg = err.to_string();
            let body: serde_json::Value = serde_json::json!({"error": error_msg});
            (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
    }
}
