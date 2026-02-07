pub mod queries;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// Create a SQLite database connection pool for development.
/// For production, use PostgreSQL by modifying this function.
///
/// # Arguments
///
/// * `database_url` - SQLite connection string (e.g., "sqlite:fishing.db")
///
/// # Returns
///
/// Returns a SQLite connection pool.
pub async fn create_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("{}?mode=rwc", database_url))
        .await?;

    // Initialize schema if needed
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS catches (
            id TEXT PRIMARY KEY,
            user_id TEXT,
            location_lat REAL NOT NULL,
            location_lon REAL NOT NULL,
            caught_at TIMESTAMP NOT NULL,
            fish_species TEXT,
            weight_kg REAL,
            bait_used TEXT,
            notes TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS water_bodies (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            location_lat REAL NOT NULL,
            location_lon REAL NOT NULL,
            water_type TEXT,
            country_code TEXT
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS fish_species (
            id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
            name_uk TEXT NOT NULL,
            name_en TEXT,
            scientific_name TEXT,
            best_season TEXT,
            preferred_bait TEXT,
            min_temp REAL,
            max_temp REAL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS regulations (
            id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
            region_code TEXT NOT NULL,
            fish_species TEXT,
            min_size_cm REAL,
            max_catch_per_day INTEGER,
            season_start TEXT,
            season_end TEXT,
            restrictions TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Database schema initialized");

    Ok(pool)
}
