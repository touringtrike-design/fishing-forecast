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
    pub id: String,
    pub name: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub water_type: Option<String>,
}

/// Fishing regulation row for API responses.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegulationDb {
    pub id: String,
    pub region_code: String,
    pub fish_species: Option<String>,
    pub min_size_cm: Option<f64>,
    pub max_catch_per_day: Option<i32>,
    pub season_start: Option<String>,
    pub season_end: Option<String>,
    pub restrictions: Option<String>,
}

/// Fish item for localized lists.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FishItemDb {
    pub id: String,
    pub name: String,
    pub scientific_name: String,
}
