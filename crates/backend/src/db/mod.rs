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

    // Ensure UTF-8 encoding
    sqlx::query("PRAGMA encoding = 'UTF-8'")
        .execute(&pool)
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

    // Seed fish species with proper UTF-8 encoding
    let fish_data = vec![
        ("pike", "\u{0429}\u{043A}\u{0443}\u{043A}\u{0430}", "Pike", "Esox lucius"),        // Щука
        ("crucian", "\u{041A}\u{0430}\u{0440}\u{0430}\u{0441}\u{044C}", "Crucian carp", "Carassius carassius"), // Карась
        ("perch", "\u{041E}\u{043A}\u{0443}\u{043D}\u{044C}", "Perch", "Perca fluviatilis"),  // Окунь
        ("bream", "\u{041B}\u{044F}\u{0449}", "Bream", "Abramis brama"),                    // Лящ
        ("zander", "\u{0421}\u{0443}\u{0434}\u{0430}\u{043A}", "Zander", "Sander lucioperca"), // Судак
        ("carp", "\u{041A}\u{043E}\u{0440}\u{043E}\u{043F}", "Carp", "Cyprinus carpio"),    // Короп
        ("catfish", "\u{0421}\u{043E}\u{043C}", "Catfish", "Silurus glanis"),               // Сом
        ("roach", "\u{041F}\u{043B}\u{0456}\u{0442}\u{043A}\u{0430}", "Roach", "Rutilus rutilus"), // Плітка
        ("common-carp", "\u{041A}\u{0430}\u{0440}\u{043F}", "Common carp", "Cyprinus carpio"), // Карп
        ("tench", "\u{041B}\u{0438}\u{043D}", "Tench", "Tinca tinca"),                    // Лин
    ];

    for (id, name_uk, name_en, scientific_name) in fish_data {
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
        .bind("all_year")
        .bind("worm")
        .bind(5.0)
        .bind(28.0)
        .execute(&pool)
        .await?;
    }

    tracing::info!("Database schema initialized");

    Ok(pool)
}
