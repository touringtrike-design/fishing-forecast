use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{db::queries, AppState};

/// Query parameters for fish species list.
#[derive(Debug, Deserialize)]
pub struct FishQuery {
    pub country: Option<String>,
    pub language: Option<String>,
}

/// Get localized fish species list.
pub async fn fish_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<FishQuery>,
) -> impl IntoResponse {
    let country = query.country.unwrap_or_else(|| "EU".to_string()).to_uppercase();
    let language = query.language.unwrap_or_else(|| "en".to_string()).to_lowercase();

    match queries::get_fish_species(&state.db, &country, &language).await {
        Ok(rows) => Json::<Vec<_>>(rows).into_response(),
        Err(err) => {
            let body: serde_json::Value = serde_json::json!({"error": err.to_string() as String});
            (axum::http::StatusCode::BAD_REQUEST, Json(body)).into_response()
        }
    }
}
