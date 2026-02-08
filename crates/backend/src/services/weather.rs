use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use fishing_shared::types::WeatherCurrent;

// ========== Open-Meteo Structures ==========
#[derive(Debug, Deserialize)]
struct OpenMeteoCurrent {
    temperature_2m: f64,
    pressure_msl: f64,
    wind_speed_10m: f64,
    wind_direction_10m: f64,
    wind_gusts_10m: Option<f64>,
    precipitation: Option<f64>,
    time: String,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    current: OpenMeteoCurrent,
}

/// Fetch current weather from Open-Meteo (безкоштовний).
pub async fn fetch_current_weather(
    http: &reqwest::Client,
    lat: f64,
    lon: f64,
) -> anyhow::Result<WeatherCurrent> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,pressure_msl,wind_speed_10m,wind_direction_10m,wind_gusts_10m,precipitation&timezone=UTC",
        lat, lon
    );

    tracing::debug!("Fetching weather from Open-Meteo: {}", url);

    let resp = http
        .get(&url)
        .send()
        .await
        .context("open-meteo request failed")?
        .error_for_status()
        .context("open-meteo error status")?
        .json::<OpenMeteoResponse>()
        .await
        .context("open-meteo json parse failed")?;

    tracing::debug!("Open-Meteo response: {:?}", resp);

    // Parse time - Open-Meteo returns simplified format like "2026-02-07T16:45"
    let time = if let Ok(parsed) = DateTime::parse_from_rfc3339(&format!("{}:00+00:00", resp.current.time)) {
        parsed.with_timezone(&Utc)
    } else if let Ok(parsed) = chrono::NaiveDateTime::parse_from_str(&resp.current.time, "%Y-%m-%dT%H:%M") {
        DateTime::<Utc>::from_naive_utc_and_offset(parsed, Utc)
    } else {
        anyhow::bail!("invalid time format: {}", resp.current.time)
    };

    Ok(WeatherCurrent {
        temperature_c: resp.current.temperature_2m,
        pressure_hpa: resp.current.pressure_msl,
        wind_speed_ms: resp.current.wind_speed_10m,
        wind_gust_ms: resp.current.wind_gusts_10m,
        wind_direction_deg: Some(resp.current.wind_direction_10m),
        precipitation_mm: resp.current.precipitation,
        time,
    })
}
