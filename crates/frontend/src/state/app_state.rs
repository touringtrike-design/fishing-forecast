use serde::{Deserialize, Serialize};

/// Global application state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    /// Currently selected location (lat, lon)
    pub selected_location: Option<(f64, f64)>,
    
    /// Current forecast data
    pub current_forecast: Option<String>, // Would store ForecastResponse
    
    /// User preferences
    pub preferences: UserPreferences,
    
    /// Whether offline mode is active
    pub offline_mode: bool,
    
    /// Pending actions to sync when online
    pub pending_syncs: Vec<String>,
}

/// User preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred language (uk, en, pl, de, fr)
    pub language: String,
    
    /// Length unit (cm or in)
    pub length_unit: String,
    
    /// Weight unit (kg or lb)
    pub weight_unit: String,
    
    /// Temperature unit (C or F)
    pub temperature_unit: String,
    
    /// Pressure unit (mmHg, hPa, inHg)
    pub pressure_unit: String,
    
    /// Dark mode enabled
    pub dark_mode: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            selected_location: None,
            current_forecast: None,
            preferences: UserPreferences::default(),
            offline_mode: false,
            pending_syncs: Vec::new(),
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "uk".to_string(),
            length_unit: "cm".to_string(),
            weight_unit: "kg".to_string(),
            temperature_unit: "C".to_string(),
            pressure_unit: "hPa".to_string(),
            dark_mode: false,
        }
    }
}
