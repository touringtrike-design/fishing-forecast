use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User catch log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchRecord {
    pub id: String,
    pub user_id: String,
    pub lat: f64,
    pub lon: f64,
    pub caught_at: DateTime<Utc>,
    pub fish_species: String,
    pub weight_kg: Option<f64>,
    pub length_cm: Option<f64>,
    pub bait_used: String,
    pub bite_intensity: Option<u8>,
    pub notes: Option<String>,
}
