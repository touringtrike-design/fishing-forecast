use serde::{Deserialize, Serialize};

use super::weather::WeatherCurrent;

/// Scoring factors used to explain the forecast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastFactors {
    pub pressure_score: f64,
    pub temperature_score: f64,
    pub time_of_day_score: f64,
    pub wind_score: f64,
    pub moon_score: f64,
    pub other_score: f64,
}

/// Recommended bait with a relative score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaitRecommendation {
    pub name: String,
    pub score: f64,
}

/// Forecast response returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResult {
    pub probability: f64,
    pub confidence: f64,
    pub factors: ForecastFactors,
    pub explanation: String,
    pub recommended_baits: Vec<BaitRecommendation>,
    pub best_time: String,
    pub weather: WeatherCurrent,
    pub moon_phase: f64,
}
