use chrono::{DateTime, Utc};

/// Calculate a simple moon phase approximation.
///
/// Returns a value in range 0.0-1.0 where 0.0 is new moon and 0.5 is full moon.
pub fn moon_phase(date: DateTime<Utc>) -> f64 {
    let known_new_moon = chrono::NaiveDate::from_ymd_opt(2000, 1, 6)
        .and_then(|d| d.and_hms_opt(18, 14, 0));

    let Some(known_new_moon) = known_new_moon else {
        return 0.0;
    };

    let date_naive = date.naive_utc();
    let days = (date_naive - known_new_moon).num_seconds() as f64 / 86_400.0;
    let synodic_month = 29.53058867_f64;
    let phase = (days / synodic_month).fract();
    if phase < 0.0 { phase + 1.0 } else { phase }
}

/// Convert a moon phase value into a human-readable name.
pub fn moon_phase_name(phase: f64) -> &'static str {
    match phase {
        p if p <= 0.03 || p >= 0.97 => "New Moon",
        p if p <= 0.22 => "Waxing Crescent",
        p if p <= 0.28 => "First Quarter",
        p if p <= 0.47 => "Waxing Gibbous",
        p if p <= 0.53 => "Full Moon",
        p if p <= 0.72 => "Waning Gibbous",
        p if p <= 0.78 => "Last Quarter",
        _ => "Waning Crescent",
    }
}

/// Estimate moon illumination percentage from phase (0.0-1.0).
pub fn moon_illumination(phase: f64) -> f64 {
    let radians = phase * std::f64::consts::TAU;
    ((1.0 - radians.cos()) / 2.0).clamp(0.0, 1.0)
}
