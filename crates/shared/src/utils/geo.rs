use anyhow::anyhow;

/// Clamp latitude to valid range (-90 to 90).
pub fn clamp_lat(lat: f64) -> f64 {
    lat.clamp(-90.0, 90.0)
}

/// Clamp longitude to valid range (-180 to 180).
pub fn clamp_lon(lon: f64) -> f64 {
    lon.clamp(-180.0, 180.0)
}

/// Calculate Haversine distance in kilometers.
pub fn distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    r * c
}

/// Convert a point to WKT string.
pub fn point_to_wkt(lat: f64, lon: f64) -> String {
    format!("POINT({} {})", lon, lat)
}

/// Parse a WKT point string into (lat, lon).
pub fn wkt_to_point(wkt: &str) -> anyhow::Result<(f64, f64)> {
    let wkt = wkt.trim();
    let prefix = "POINT(";
    let suffix = ")";
    if !wkt.starts_with(prefix) || !wkt.ends_with(suffix) {
        return Err(anyhow!("invalid WKT point"));
    }

    let coords = &wkt[prefix.len()..wkt.len() - suffix.len()];
    let mut parts = coords.split_whitespace();
    let lon = parts
        .next()
        .ok_or_else(|| anyhow!("invalid WKT point"))?
        .parse::<f64>()?;
    let lat = parts
        .next()
        .ok_or_else(|| anyhow!("invalid WKT point"))?
        .parse::<f64>()?;

    Ok((lat, lon))
}
