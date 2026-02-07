use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{CatchRecordDb, FishItemDb, NewCatchRecord, RegulationDb, WaterBodyDb};

/// Insert a new catch record and return the saved row.
pub async fn insert_catch(
    pool: &SqlitePool,
    record: &NewCatchRecord,
) -> anyhow::Result<CatchRecordDb> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query(
        r#"
        INSERT INTO catches (
            id, user_id, location_lat, location_lon, caught_at,
            fish_species, weight_kg, length_cm, bait_used, weather_temp,
            weather_pressure, moon_phase, notes, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id.to_string())
    .bind(record.user_id.to_string())
    .bind(record.lat)
    .bind(record.lon)
    .bind(&record.caught_at)
    .bind(&record.fish_species)
    .bind(record.weight)
    .bind(record.length)
    .bind(&record.bait_used)
    .bind(record.weather_temp)
    .bind(record.weather_pressure)
    .bind(record.moon_phase)
    .bind(&record.notes)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(CatchRecordDb {
        id,
        user_id: record.user_id,
        lat: record.lat,
        lon: record.lon,
        caught_at: record.caught_at,
        fish_species: record.fish_species.clone(),
        weight: record.weight,
        length: record.length,
        bait_used: record.bait_used.clone(),
        weather_temp: record.weather_temp,
        weather_pressure: record.weather_pressure,
        moon_phase: record.moon_phase,
        notes: record.notes.clone(),
    })
}

/// Get catches by user ID with pagination.
pub async fn get_catches_by_user(
    pool: &SqlitePool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<CatchRecordDb>> {
    let rows = sqlx::query_as::<_, CatchRecordDb>(
        r#"
        SELECT id, user_id, location_lat, location_lon, caught_at,
               fish_species, weight_kg, bait_used, notes
        FROM catches
        WHERE user_id = ?
        ORDER BY caught_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user_id.to_string())
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get nearby catches within a radius in kilometers.
pub async fn get_nearby_catches(
    pool: &SqlitePool,
    lat: f64,
    lon: f64,
    radius_km: f64,
) -> anyhow::Result<Vec<CatchRecordDb>> {
    let lat_delta = radius_km / 111.0;
    let lon_delta = radius_km / (111.0 * (lat * std::f64::consts::PI / 180.0).cos());

    let rows = sqlx::query_as::<_, CatchRecordDb>(
        r#"
        SELECT id, user_id, location_lat, location_lon, caught_at,
               fish_species, weight_kg, bait_used, notes
        FROM catches
        WHERE location_lat BETWEEN ? AND ?
          AND location_lon BETWEEN ? AND ?
        ORDER BY caught_at DESC
        LIMIT 100
        "#,
    )
    .bind(lat - lat_delta)
    .bind(lat + lat_delta)
    .bind(lon - lon_delta)
    .bind(lon + lon_delta)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get nearby water bodies within a radius in kilometers.
pub async fn get_nearby_water_bodies(
    pool: &SqlitePool,
    lat: f64,
    lon: f64,
    radius_km: f64,
) -> anyhow::Result<Vec<WaterBodyDb>> {
    let lat_delta = radius_km / 111.0;
    let lon_delta = radius_km / (111.0 * (lat * std::f64::consts::PI / 180.0).cos());

    let rows = sqlx::query_as::<_, WaterBodyDb>(
        r#"
        SELECT id, name, location_lat, location_lon, water_type
        FROM water_bodies
        WHERE location_lat BETWEEN ? AND ?
          AND location_lon BETWEEN ? AND ?
        ORDER BY
            (location_lat - ?) * (location_lat - ?) +
            (location_lon - ?) * (location_lon - ?) ASC
        LIMIT 200
        "#,
    )
    .bind(lat - lat_delta)
    .bind(lat + lat_delta)
    .bind(lon - lon_delta)
    .bind(lon + lon_delta)
    .bind(lat)
    .bind(lat)
    .bind(lon)
    .bind(lon)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get regulations for a country and optional fish species.
pub async fn get_regulations(
    pool: &SqlitePool,
    country_code: &str,
    fish_species_id: Option<Uuid>,
) -> anyhow::Result<Vec<RegulationDb>> {
    let rows = if let Some(_fish_id) = fish_species_id {
        sqlx::query_as::<_, RegulationDb>(
            r#"
            SELECT id, region_code, fish_species, min_size_cm, max_catch_per_day,
                   season_start, season_end, restrictions
            FROM regulations
            WHERE region_code = ?
            "#,
        )
        .bind(country_code)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, RegulationDb>(
            r#"
            SELECT id, region_code, fish_species, min_size_cm, max_catch_per_day,
                   season_start, season_end, restrictions
            FROM regulations
            WHERE region_code = ?
            "#,
        )
        .bind(country_code)
        .fetch_all(pool)
        .await?
    };

    Ok(rows)
}

/// Get localized fish list for a country and language.
pub async fn get_fish_species(
    pool: &SqlitePool,
    _country_code: &str,
    _language: &str,
) -> anyhow::Result<Vec<FishItemDb>> {
    tracing::debug!("Querying fish_species table");
    
    // Query fish_species table directly
    let rows = sqlx::query_as::<_, FishItemDb>(
        r#"
        SELECT id, name_uk as name, scientific_name
        FROM fish_species
        ORDER BY name_uk
        "#,
    )
    .fetch_all(pool)
    .await?;

    tracing::debug!("Found {} fish species", rows.len());
    
    Ok(rows)
}
