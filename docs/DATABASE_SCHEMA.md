# Database Schema

This document describes the PostgreSQL 15 + PostGIS schema used by Fishing Forecast.

## Extensions

```sql
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

## Core Tables

### users

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key, default `uuid_generate_v4()` |
| email | TEXT | Unique, optional |
| country_code | TEXT | ISO country code |
| language | TEXT | Language code |
| unit_system | TEXT | Metric/Imperial |
| created_at | TIMESTAMPTZ | Default `NOW()` |

### catches

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key |
| user_id | UUID | FK → users(id) |
| location | GEOGRAPHY(POINT, 4326) | Required |
| caught_at | TIMESTAMPTZ | Required |
| fish_species | TEXT | Required |
| weight | DOUBLE PRECISION | Optional |
| length | DOUBLE PRECISION | Optional |
| bait_used | TEXT | Required |
| weather_temp | DOUBLE PRECISION | Optional |
| weather_pressure | DOUBLE PRECISION | Optional |
| moon_phase | DOUBLE PRECISION | Optional |
| notes | TEXT | Optional |

Indexes:
- `idx_catches_location` (GIST on `location`)
- `idx_catches_user_id` (BTREE on `user_id`)

### water_bodies

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key |
| name | TEXT | Required |
| water_type | TEXT | Optional |
| location | GEOGRAPHY(POINT, 4326) | Optional |
| geometry | GEOGRAPHY(GEOMETRY, 4326) | Optional |
| area_sqm | DOUBLE PRECISION | Optional |
| cached_at | TIMESTAMPTZ | Optional |

Indexes:
- `idx_water_bodies_location` (GIST on `location`)

## i18n Tables

### countries

| Column | Type | Notes |
|---|---|---|
| code | TEXT | Primary key |
| name_en | TEXT | Required |
| name_local | TEXT | Optional |
| region | TEXT | Optional |
| currency | TEXT | Optional |
| supported | BOOLEAN | Default `FALSE` |

### languages

| Column | Type | Notes |
|---|---|---|
| code | TEXT | Primary key |
| name_en | TEXT | Required |
| name_local | TEXT | Optional |
| active | BOOLEAN | Default `TRUE` |

### fish_species

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key |
| scientific_name | TEXT | Required |
| family | TEXT | Optional |
| habitat | TEXT | Optional |

### fish_names

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key |
| fish_id | UUID | FK → fish_species(id) |
| language | TEXT | Required |
| common_name | TEXT | Required |

### fish_regions

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key |
| fish_id | UUID | FK → fish_species(id) |
| country_code | TEXT | Required |
| abundance | TEXT | Optional |
| notes | TEXT | Optional |

Indexes:
- `idx_fish_regions_country`
- `idx_fish_names_language`

## Regulations Tables

### fishing_regulations

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key |
| country_code | TEXT | Required |
| region | TEXT | Optional |
| fish_species_id | UUID | FK → fish_species(id) |
| license_required | BOOLEAN | Default `FALSE` |
| license_cost_local | TEXT | Optional |
| license_url | TEXT | Optional |
| min_size_cm | DOUBLE PRECISION | Optional |
| max_size_cm | DOUBLE PRECISION | Optional |
| daily_limit | INTEGER | Optional |
| closed_season_start | DATE | Optional |
| closed_season_end | DATE | Optional |
| protected | BOOLEAN | Default `FALSE` |
| notes | TEXT | Optional |

Indexes:
- `idx_regulations_country`
- `idx_regulations_species`

### prohibited_gear

| Column | Type | Notes |
|---|---|---|
| id | UUID | Primary key |
| country_code | TEXT | Required |
| region | TEXT | Optional |
| gear_type | TEXT | Required |
| description | TEXT | Optional |
| exceptions | TEXT | Optional |

## Stored Procedures

### get_nearby_catches(lat, lon, radius_km)

Find catches within a radius using PostGIS `ST_DWithin`.

### get_nearby_water_bodies(lat, lon, radius_km)

Find water bodies within a radius using PostGIS `ST_DWithin`.
