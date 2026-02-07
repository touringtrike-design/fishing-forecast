-- Create migrations directory schema for SQLite

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE,
    country_code TEXT,
    language TEXT DEFAULT 'uk',
    unit_system TEXT DEFAULT 'metric',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Water bodies table
CREATE TABLE IF NOT EXISTS water_bodies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    location_lat REAL NOT NULL,
    location_lon REAL NOT NULL,
    water_type TEXT, -- 'river', 'lake', 'pond'
    country_code TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Fish species table
CREATE TABLE IF NOT EXISTS fish_species (
    id TEXT PRIMARY KEY,
    name_uk TEXT NOT NULL,
    name_en TEXT,
    scientific_name TEXT,
    best_season TEXT, -- 'spring', 'summer', 'autumn', 'winter'
    preferred_bait TEXT,
    min_temp REAL,
    max_temp REAL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Catches (records of caught fish)
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
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Regulations by region
CREATE TABLE IF NOT EXISTS regulations (
    id TEXT PRIMARY KEY,
    region_code TEXT NOT NULL,
    fish_species TEXT,
    min_size_cm REAL,
    max_catch_per_day INTEGER,
    season_start TEXT, -- YYYY-MM-DD
    season_end TEXT,   -- YYYY-MM-DD
    restrictions TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_catches_user_id ON catches(user_id);
CREATE INDEX IF NOT EXISTS idx_catches_location ON catches(location_lat, location_lon);
CREATE INDEX IF NOT EXISTS idx_water_bodies_country ON water_bodies(country_code);
CREATE INDEX IF NOT EXISTS idx_regulations_region ON regulations(region_code);
CREATE INDEX IF NOT EXISTS idx_catches_caught_at ON catches(caught_at);
