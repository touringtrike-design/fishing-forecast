use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Current weather snapshot used for forecasts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherCurrent {
    pub temperature_c: f64,
    pub pressure_hpa: f64,
    pub wind_speed_ms: f64,
    pub wind_gust_ms: Option<f64>,
    pub wind_direction_deg: Option<f64>,
    pub precipitation_mm: Option<f64>,
    pub time: DateTime<Utc>,
}
