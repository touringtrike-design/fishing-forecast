use serde::{Deserialize, Serialize};

/// Water body representation returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterBody {
    pub id: String,
    pub name: String,
    pub water_type: String,
    pub lat: f64,
    pub lon: f64,
    pub area_sqm: Option<f64>,
}
