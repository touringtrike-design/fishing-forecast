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
            let items: Vec<RegulationItem> = rows
                .into_iter()
                .map(|row: _| RegulationItem {
                    license_required: row.license_required,
                    license_cost: row.license_cost_local,
                    license_url: row.license_url,
                    min_size_cm: row.min_size_cm,
                    max_size_cm: row.max_size_cm,
                    daily_limit: row.daily_limit.map(|v| v as u32),
                    closed_season: match (row.closed_season_start, row.closed_season_end) {
                        (Some(start), Some(end)) => {
                            Some(format!("{} to {}", start, end))
                        }
                        _ => None,
                    },
                    protected_species: Vec::new(),
                    prohibited_gear: Vec::new(),
                })
                .collect::<Vec<_>>();

            Json(items).into_response()
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
            if let Some(max_size) = rule.max_size_cm {
                if size_cm > max_size {
                    warnings.push(format!("Size above maximum ({max_size} cm)"));
                }
            }
        }
    }

    if let Some(date) = payload.date {
        for rule in &regulations {
            if let (Some(start), Some(end)) = (rule.closed_season_start, rule.closed_season_end) {
                if date >= start && date <= end {
                    errors.push("Fishing during closed season".to_string());
                }
            }
        }
    }

    for rule in &regulations {
        if rule.protected {
            errors.push("Protected species".to_string());
        }
        if rule.license_required {
            warnings.push("License required".to_string());
        }
    }

    let allowed = errors.is_empty();
    Json(ValidationResult { allowed, errors, warnings }).into_response()
}
