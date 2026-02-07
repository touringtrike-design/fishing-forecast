use anyhow::Context;
use serde::Deserialize;

use fishing_shared::types::RegionInfo;

#[derive(Debug, Deserialize)]
struct NominatimAddress {
    country_code: Option<String>,
    country: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NominatimResponse {
    address: Option<NominatimAddress>,
}

/// Detect country and support status using Nominatim reverse geocoding.
pub async fn detect_region(
    http: &reqwest::Client,
    lat: f64,
    lon: f64,
) -> anyhow::Result<RegionInfo> {
    let url = format!(
        "https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat={lat}&lon={lon}",
    );

    let resp = http
        .get(url)
        .send()
        .await
        .context("nominatim request failed")?
        .error_for_status()
        .context("nominatim error status")?
        .json::<NominatimResponse>()
        .await
        .context("nominatim json parse failed")?;

    let country_code = resp
        .address
        .as_ref()
        .and_then(|a| a.country_code.clone())
        .unwrap_or_else(|| "EU".to_string())
        .to_uppercase();

    let country_name = resp
        .address
        .as_ref()
        .and_then(|a| a.country.clone())
        .unwrap_or_else(|| "European Union".to_string());

    let supported = matches!(country_code.as_str(), "UA" | "PL" | "EU");

    Ok(RegionInfo {
        country_code,
        country_name,
        supported,
    })
}
