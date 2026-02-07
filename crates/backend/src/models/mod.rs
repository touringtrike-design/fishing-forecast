use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Catch record stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CatchRecordDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub lat: f64,
    pub lon: f64,
    pub caught_at: DateTime<Utc>,
    pub fish_species: String,
    pub weight: Option<f64>,
    pub length: Option<f64>,
    pub bait_used: String,
    pub weather_temp: Option<f64>,
    pub weather_pressure: Option<f64>,
    pub moon_phase: Option<f64>,
    pub notes: Option<String>,
}

/// New catch record payload (before insert).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCatchRecord {
    pub user_id: Uuid,
    pub lat: f64,
    pub lon: f64,
    pub caught_at: DateTime<Utc>,
    pub fish_species: String,
    pub weight: Option<f64>,
    pub length: Option<f64>,
    pub bait_used: String,
    pub weather_temp: Option<f64>,
    pub weather_pressure: Option<f64>,
    pub moon_phase: Option<f64>,
    pub notes: Option<String>,
}

/// Water body stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WaterBodyDb {
    pub id: Uuid,
    pub name: String,
    pub water_type: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub area_sqm: Option<f64>,
    pub cached_at: Option<DateTime<Utc>>,
}

/// Fishing regulation row for API responses.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegulationDb {
    pub id: Uuid,
    pub country_code: String,
    pub region: Option<String>,
    pub fish_species_id: Option<Uuid>,
    pub license_required: bool,
    pub license_cost_local: Option<String>,
    pub license_url: Option<String>,
    pub min_size_cm: Option<f64>,
    pub max_size_cm: Option<f64>,
    pub daily_limit: Option<i32>,
    pub closed_season_start: Option<chrono::NaiveDate>,
    pub closed_season_end: Option<chrono::NaiveDate>,
    pub protected: bool,
    pub notes: Option<String>,
}

/// Fish item for localized lists.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FishItemDb {
    pub id: Uuid,
    pub name: String,
    pub scientific_name: String,
}
