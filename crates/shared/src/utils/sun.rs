use chrono::{DateTime, Utc};

/// Sunrise and sunset times in UTC.
#[derive(Debug, Clone)]
pub struct SunTimes {
    pub sunrise_utc: DateTime<Utc>,
    pub sunset_utc: DateTime<Utc>,
}

/// Estimate sunrise and sunset times.
///
/// NOTE: This is a placeholder. Replace with an accurate astronomical
/// calculation or a dedicated crate (e.g., suncalc) when implementing UI.
pub fn estimate_sun_times(_lat: f64, _lon: f64, date: DateTime<Utc>) -> SunTimes {
    SunTimes {
        sunrise_utc: date,
        sunset_utc: date,
    }
}
