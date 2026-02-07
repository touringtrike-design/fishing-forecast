use axum::{extract::Query, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Query parameters for regulations lookup.
#[derive(Debug, Deserialize)]
pub struct RegulationsQuery {
    pub country: String,
    pub fish: Option<String>,
}

/// Regulations response item.
#[derive(Debug, Serialize)]
pub struct RegulationItem {
    pub license_required: bool,
    pub license_cost: Option<String>,
    pub license_url: Option<String>,
    pub min_size_cm: Option<f64>,
    pub max_size_cm: Option<f64>,
    pub daily_limit: Option<u32>,
    pub closed_season: Option<String>,
    pub protected_species: Vec<String>,
    pub prohibited_gear: Vec<String>,
}

/// Request payload for regulations validation.
#[derive(Debug, Deserialize)]
pub struct ValidationRequest {
    pub country_code: String,
    pub fish_species: Option<String>,
    pub size_cm: Option<f64>,
    pub date: Option<NaiveDate>,
}

/// Validation result for a catch.
#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub allowed: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Get regulations for a given country and optional fish.
pub async fn regulations_handler(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    Query(query): Query<RegulationsQuery>,
) -> impl IntoResponse {
    let country = query.country.to_uppercase();
    let fish_species_id = query
        .fish
        .as_ref()
        .and_then(|id| Uuid::parse_str(id).ok());

    match crate::db::queries::get_regulations(&state.db, &country, fish_species_id).await {
        Ok(rows) => {
            // Simplified response for Phase 1 - just return raw database rows
            Json(rows).into_response()
        }
        Err(err) => {
            let error_msg = err.to_string();
            let body: serde_json::Value = serde_json::json!({"error": error_msg});
            (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
    }
}

/// Validate a catch against regulations.
pub async fn regulations_validate_handler(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    Json(payload): Json<ValidationRequest>,
) -> impl IntoResponse {
    let country = payload.country_code.to_uppercase();
    let fish_species_id = payload
        .fish_species
        .as_ref()
        .and_then(|id| Uuid::parse_str(id).ok());

    let regulations = match crate::db::queries::get_regulations(&state.db, &country, fish_species_id).await {
        Ok(rows) => rows,
        Err(err) => {
            let error_msg = err.to_string();
            let body: serde_json::Value = serde_json::json!({"error": error_msg});
            return (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response();
        }
    };

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if let Some(size_cm) = payload.size_cm {
        for rule in &regulations {
            if let Some(min_size) = rule.min_size_cm {
                if size_cm < min_size {
                    errors.push(format!("Size below minimum ({min_size} cm)"));
                }
            }
            // max_size_cm field not in RegulationDb - skip for Phase 1
        }
    }

    // Simplified validation for Phase 1 - skip date and protection checks
    // as those fields don't exist in current RegulationDb model

    let allowed = errors.is_empty();
    Json(ValidationResult { allowed, errors, warnings }).into_response()
}
