CREATE TABLE IF NOT EXISTS countries (
    code TEXT PRIMARY KEY,
    name_en TEXT NOT NULL,
    name_local TEXT,
    region TEXT,
    currency TEXT,
    supported BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS languages (
    code TEXT PRIMARY KEY,
    name_en TEXT NOT NULL,
    name_local TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS fish_species (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scientific_name TEXT NOT NULL,
    family TEXT,
    habitat TEXT
);

CREATE TABLE IF NOT EXISTS fish_names (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fish_id UUID REFERENCES fish_species(id),
    language TEXT NOT NULL,
    common_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fish_regions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fish_id UUID REFERENCES fish_species(id),
    country_code TEXT NOT NULL,
    abundance TEXT,
    notes TEXT
);
