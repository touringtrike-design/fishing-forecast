CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT UNIQUE,
    country_code TEXT,
    language TEXT,
    unit_system TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS catches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    location GEOGRAPHY(POINT, 4326) NOT NULL,
    caught_at TIMESTAMPTZ NOT NULL,
    fish_species TEXT NOT NULL,
    weight DOUBLE PRECISION,
    length DOUBLE PRECISION,
    bait_used TEXT NOT NULL,
    weather_temp DOUBLE PRECISION,
    weather_pressure DOUBLE PRECISION,
    moon_phase DOUBLE PRECISION,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS water_bodies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    water_type TEXT,
    location GEOGRAPHY(POINT, 4326),
    geometry GEOGRAPHY(GEOMETRY, 4326),
    area_sqm DOUBLE PRECISION,
    cached_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_catches_location ON catches USING GIST(location);
CREATE INDEX IF NOT EXISTS idx_catches_user_id ON catches(user_id);
CREATE INDEX IF NOT EXISTS idx_water_bodies_location ON water_bodies USING GIST(location);

CREATE OR REPLACE FUNCTION get_nearby_catches(
    lat DOUBLE PRECISION,
    lon DOUBLE PRECISION,
    radius_km DOUBLE PRECISION
) RETURNS TABLE (
    id UUID,
    user_id UUID,
    location GEOGRAPHY(POINT, 4326),
    caught_at TIMESTAMPTZ,
    fish_species TEXT,
    weight DOUBLE PRECISION,
    length DOUBLE PRECISION,
    bait_used TEXT,
    weather_temp DOUBLE PRECISION,
    weather_pressure DOUBLE PRECISION,
    moon_phase DOUBLE PRECISION,
    notes TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        c.id,
        c.user_id,
        c.location,
        c.caught_at,
        c.fish_species,
        c.weight,
        c.length,
        c.bait_used,
        c.weather_temp,
        c.weather_pressure,
        c.moon_phase,
        c.notes
    FROM catches c
    WHERE ST_DWithin(
        c.location,
        ST_SetSRID(ST_MakePoint(lon, lat), 4326)::geography,
        radius_km * 1000.0
    );
END;
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_nearby_water_bodies(
    lat DOUBLE PRECISION,
    lon DOUBLE PRECISION,
    radius_km DOUBLE PRECISION
) RETURNS TABLE (
    id UUID,
    name TEXT,
    water_type TEXT,
    location GEOGRAPHY(POINT, 4326),
    geometry GEOGRAPHY(GEOMETRY, 4326),
    area_sqm DOUBLE PRECISION,
    cached_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        w.id,
        w.name,
        w.water_type,
        w.location,
        w.geometry,
        w.area_sqm,
        w.cached_at
    FROM water_bodies w
    WHERE (w.location IS NOT NULL AND ST_DWithin(
        w.location,
        ST_SetSRID(ST_MakePoint(lon, lat), 4326)::geography,
        radius_km * 1000.0
    )) OR (w.geometry IS NOT NULL AND ST_DWithin(
        w.geometry,
        ST_SetSRID(ST_MakePoint(lon, lat), 4326)::geography,
        radius_km * 1000.0
    ));
END;
$$ LANGUAGE plpgsql STABLE;
