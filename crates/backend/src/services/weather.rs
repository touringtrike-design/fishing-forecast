use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use fishing_shared::types::WeatherCurrent;

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

/// Fetch current weather from Open-Meteo.
pub async fn fetch_current_weather(
    http: &reqwest::Client,
    lat: f64,
    lon: f64,
) -> anyhow::Result<WeatherCurrent> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,pressure_msl,wind_speed_10m,wind_direction_10m,wind_gusts_10m,precipitation&timezone=UTC",
        lat, lon
    );

    tracing::debug!("Fetching weather from: {}", url);

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

    tracing::debug!("Weather response: {:?}", resp);

    let time = DateTime::parse_from_rfc3339(&resp.current.time)
        .context("invalid open-meteo time")?
        .with_timezone(&Utc);

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
