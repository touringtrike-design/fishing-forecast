use anyhow::Context;
use chrono::Timelike;
use fishing_shared::{types::{BaitRecommendation, ForecastFactors, ForecastResult}, utils::moon::moon_phase};

use super::weather::fetch_current_weather;

fn clamp01(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

fn score_pressure(pressure_hpa: f64) -> f64 {
    if (1000.0..=1020.0).contains(&pressure_hpa) {
        0.9
    } else if (990.0..=1030.0).contains(&pressure_hpa) {
        0.6
    } else {
        0.3
    }
}

fn score_temperature(temp_c: f64) -> f64 {
    if (15.0..=22.0).contains(&temp_c) {
        0.85
    } else if (10.0..=28.0).contains(&temp_c) {
        0.55
    } else {
        0.25
    }
}

fn score_time_of_day(hour: u32) -> f64 {
    if (5..=9).contains(&hour) || (18..=21).contains(&hour) {
        0.8
    } else if (11..=16).contains(&hour) {
        0.3
    } else {
        0.5
    }
}

fn score_wind(wind_ms: f64) -> f64 {
    if (2.0..=7.0).contains(&wind_ms) {
        0.8
    } else if wind_ms < 2.0 {
        0.4
    } else if wind_ms <= 12.0 {
        0.55
    } else {
        0.2
    }
}

fn score_moon(phase: f64) -> f64 {
    if phase <= 0.1 || phase >= 0.9 {
        0.8
    } else if (0.4..=0.6).contains(&phase) {
        0.6
    } else {
        0.5
    }
}

/// Build a forecast using current weather and lunar phase.
pub async fn build_forecast(
    http: &reqwest::Client,
    lat: f64,
    lon: f64,
    _fish: Option<&str>,
) -> anyhow::Result<ForecastResult> {
    let weather = fetch_current_weather(http, lat, lon)
        .await
        .context("weather fetch failed")?;

    let hour = weather.time.hour() as u32;
    let moon = moon_phase(weather.time);

    let pressure_score = score_pressure(weather.pressure_hpa);
    let temperature_score = score_temperature(weather.temperature_c);
    let time_of_day_score = score_time_of_day(hour);
    let wind_score = score_wind(weather.wind_speed_ms);
    let moon_score = score_moon(moon);
    let other_score = 0.5_f64;

    let probability = clamp01(
        0.5
            + (pressure_score * 0.40)
            + (temperature_score * 0.25)
            + (time_of_day_score * 0.15)
            + (wind_score * 0.10)
            + (moon_score * 0.05)
            + (other_score * 0.05),
    );

    let factors = ForecastFactors {
        pressure_score,
        temperature_score,
        time_of_day_score,
        wind_score,
        moon_score,
        other_score,
    };

    let recommended_baits = vec![
        BaitRecommendation { name: "worm".to_string(), score: 0.7 },
        BaitRecommendation { name: "spinner".to_string(), score: 0.6 },
        BaitRecommendation { name: "corn".to_string(), score: 0.5 },
    ];

    Ok(ForecastResult {
        probability,
        confidence: 0.4,
        factors,
        explanation: "Calculated from current weather and lunar phase.".to_string(),
        recommended_baits,
        best_time: "dawn or dusk".to_string(),
        weather,
        moon_phase: moon,
    })
}
