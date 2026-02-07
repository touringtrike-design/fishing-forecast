use serde::{Deserialize, Serialize};

/// Length units used in the UI and API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LengthUnit {
    Cm,
    Inches,
}

/// Weight units used in the UI and API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeightUnit {
    Kg,
    Lb,
}

/// Temperature units used in the UI and API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemperatureUnit {
    C,
    F,
}

/// Pressure units used in the UI and API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PressureUnit {
    Hpa,
    MmHg,
    InHg,
}

/// User preferences for units.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitPreferences {
    pub length: LengthUnit,
    pub weight: WeightUnit,
    pub temperature: TemperatureUnit,
    pub pressure: PressureUnit,
}

impl Default for UnitPreferences {
    fn default() -> Self {
        Self {
            length: LengthUnit::Cm,
            weight: WeightUnit::Kg,
            temperature: TemperatureUnit::C,
            pressure: PressureUnit::Hpa,
        }
    }
}

/// Convert centimeters to inches.
pub fn cm_to_inches(cm: f64) -> f64 {
    cm / 2.54
}

/// Convert inches to centimeters.
pub fn inches_to_cm(inches: f64) -> f64 {
    inches * 2.54
}

/// Convert kilograms to pounds.
pub fn kg_to_lb(kg: f64) -> f64 {
    kg * 2.204_622_621_8
}

/// Convert pounds to kilograms.
pub fn lb_to_kg(lb: f64) -> f64 {
    lb / 2.204_622_621_8
}

/// Convert Celsius to Fahrenheit.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

/// Convert Fahrenheit to Celsius.
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Convert millimeters of mercury to hectopascals.
pub fn mmhg_to_hpa(mmhg: f64) -> f64 {
    mmhg * 1.333_223_874_15
}

/// Convert hectopascals to millimeters of mercury.
pub fn hpa_to_mmhg(hpa: f64) -> f64 {
    hpa / 1.333_223_874_15
}

/// Convert millimeters of mercury to inches of mercury.
pub fn mmhg_to_inhg(mmhg: f64) -> f64 {
    mmhg / 25.4
}
