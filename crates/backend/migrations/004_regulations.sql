CREATE TABLE IF NOT EXISTS fishing_regulations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_code TEXT NOT NULL,
    region TEXT,
    fish_species_id UUID REFERENCES fish_species(id),
    license_required BOOLEAN NOT NULL DEFAULT FALSE,
    license_cost_local TEXT,
    license_url TEXT,
    min_size_cm DOUBLE PRECISION,
    max_size_cm DOUBLE PRECISION,
    daily_limit INTEGER,
    closed_season_start DATE,
    closed_season_end DATE,
    protected BOOLEAN NOT NULL DEFAULT FALSE,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS prohibited_gear (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_code TEXT NOT NULL,
    region TEXT,
    gear_type TEXT NOT NULL,
    description TEXT,
    exceptions TEXT
);

CREATE INDEX IF NOT EXISTS idx_regulations_country ON fishing_regulations(country_code);
CREATE INDEX IF NOT EXISTS idx_regulations_species ON fishing_regulations(fish_species_id);
