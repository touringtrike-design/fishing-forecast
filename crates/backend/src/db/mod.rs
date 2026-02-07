pub mod queries;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
#[cfg(feature = "shuttle")]
use sqlx::{postgres::PgPoolOptions, PgPool};

/// Create a SQLite database connection pool for development.
#[cfg(not(feature = "shuttle"))]
pub async fn create_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("{}?mode=rwc", database_url))
        .await?;

    // Ensure UTF-8 encoding
    sqlx::query("PRAGMA encoding = 'UTF-8'")
        .execute(&pool)
        .await?;

    // Initialize schema if needed
    initialize_schema(&pool).await?;

    tracing::info!("Database schema initialized (SQLite)");

    Ok(pool)
}

/// Create a PostgreSQL database connection pool for production (Shuttle).
#[cfg(feature = "shuttle")]
pub async fn create_pg_pool(database_url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to PostgreSQL");

    pool
}

/// Initialize database schema
async fn initialize_schema(pool: &SqlitePool) -> anyhow::Result<()> {
    // Create tables...
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE,
            password_hash TEXT NOT NULL,
            country_code TEXT DEFAULT 'UA',
            language TEXT DEFAULT 'uk',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
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
            length_cm REAL,
            bait_used TEXT,
            weather_temp REAL,
            weather_pressure REAL,
            moon_phase REAL,
            notes TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
    .await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_catches_user_id ON catches(user_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_catches_location ON catches(location_lat, location_lon)")
        .execute(pool)
        .await?;

    // Seed fish species
    seed_fish_species(pool).await?;

    Ok(())
}

/// Seed fish species data
async fn seed_fish_species(pool: &SqlitePool) -> anyhow::Result<()> {
    let fish_data = vec![
        ("pike", "Щука", "Pike", "Esox lucius", "all_year", "spoon", 5.0, 22.0),
        ("crucian", "Карась", "Crucian carp", "Carassius carassius", "summer", "worm", 15.0, 28.0),
        ("perch", "Окунь", "Perch", "Perca fluviatilis", "all_year", "minnow", 8.0, 25.0),
        ("bream", "Лящ", "Bream", "Abramis brama", "all_year", "corn", 10.0, 26.0),
        ("zander", "Судак", "Zander", "Sander lucioperca", "all_year", "fish", 12.0, 24.0),
        ("carp", "Короп", "Carp", "Cyprinus carpio", "summer", "boilie", 18.0, 30.0),
        ("catfish", "Сом", "Catfish", "Silurus glanis", "summer", "fish", 18.0, 28.0),
        ("roach", "Плітка", "Roach", "Rutilus rutilus", "all_year", "maggot", 8.0, 24.0),
        ("tench", "Лин", "Tench", "Tinca tinca", "summer", "worm", 15.0, 26.0),
        ("asp", "Жерех", "Asp", "Aspius aspius", "spring", "spoon", 10.0, 22.0),
        ("burbot", "Минь", "Burbot", "Lota lota", "winter", "lamprey", 0.0, 15.0),
        ("ide", "Язь", "Ide", "Leuciscus idus", "spring", "worm", 8.0, 22.0),
    ];

    for (id, name_uk, name_en, scientific_name, best_season, preferred_bait, min_temp, max_temp) in fish_data {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO fish_species (id, name_uk, name_en, scientific_name, best_season, preferred_bait, min_temp, max_temp)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(name_uk)
        .bind(name_en)
        .bind(scientific_name)
        .bind(best_season)
        .bind(preferred_bait)
        .bind(min_temp)
        .bind(max_temp)
        .execute(pool)
        .await?;
    }

    Ok(())
}
