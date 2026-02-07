-- Complete initialization and seeding script
-- Creates all tables and loads seed data

-- Create tables
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS catches (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    location_lat REAL NOT NULL,
    location_lon REAL NOT NULL,
    caught_at TIMESTAMP NOT NULL,
    fish_species TEXT NOT NULL,
    weight_kg REAL,
    bait_used TEXT,
    notes TEXT,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS water_bodies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    location_lat REAL NOT NULL,
    location_lon REAL NOT NULL,
    water_type TEXT,
    country_code TEXT
);

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
);

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
);

-- Insert fish species (Ukrainian fishing)
INSERT INTO fish_species (id, name_uk, name_en, scientific_name, best_season, preferred_bait, min_temp, max_temp) 
VALUES 
    ('pike', 'Щука', 'Pike', 'Esox lucius', 'spring,autumn', 'spinner,live_bait', 5.0, 18.0),
    ('crucian', 'Карась', 'Crucian carp', 'Carassius carassius', 'spring,summer', 'corn,worm,bread', 15.0, 28.0),
    ('perch', 'Окунь', 'Perch', 'Perca fluviatilis', 'all_year', 'worm,spinner', 4.0, 20.0),
    ('bream', 'Лящ', 'Bream', 'Abramis brama', 'spring,summer', 'worm,corn,pea', 12.0, 24.0),
    ('zander', 'Судак', 'Zander', 'Sander lucioperca', 'spring,autumn', 'live_bait,jig', 8.0, 18.0),
    ('carp', 'Короп', 'Carp', 'Cyprinus carpio', 'summer', 'boilies,corn,potato', 18.0, 28.0),
    ('catfish', 'Сом', 'Catfish', 'Silurus glanis', 'summer', 'live_bait,squid', 20.0, 30.0),
    ('roach', 'Плітка', 'Roach', 'Rutilus rutilus', 'all_year', 'maggot,bread,corn', 5.0, 22.0),
    ('common-carp', 'Карп', 'Common carp', 'Cyprinus carpio', 'spring,summer', 'corn,boilies', 15.0, 28.0),
    ('tench', 'Лин', 'Tench', 'Tinca tinca', 'summer', 'worm,corn', 18.0, 26.0);

-- Insert water bodies (major Ukrainian fishing spots)
INSERT INTO water_bodies (id, name, description, location_lat, location_lon, water_type, country_code)
VALUES
    ('dnipro-kyiv', 'Дніпро (Київ)', 'Головна річка України, відмінна риболовля', 50.4501, 30.5234, 'river', 'UA'),
    ('kakhovske', 'Каховське водосховище', 'Велике водосховище з багатою рибою', 47.3547, 33.6389, 'lake', 'UA'),
    ('desna', 'Десна (Чернігів)', 'Ліва притока Дніпра, щука та окунь', 51.4939, 31.2947, 'river', 'UA'),
    ('svityaz', 'Світязь', 'Найглибше озеро України', 51.4817, 23.8483, 'lake', 'UA'),
    ('pivdennyi-bug', 'Південний Буг', 'Річка з багатим рибним світом', 48.5064, 32.2628, 'river', 'UA'),
    ('kremenchuk', 'Кременчуцьке водосховище', 'Велике водосховище для трофейної риби', 49.0698, 33.4158, 'lake', 'UA'),
    ('samara', 'Самара (Новомосковськ)', 'Ліва притока Дніпра, короп та лящ', 48.6337, 35.2458, 'river', 'UA'),
    ('dnister', 'Дністер', 'Велика річка з різноманітною рибою', 48.8671, 26.4953, 'river', 'UA');

-- Insert regulations (Ukraine)
INSERT INTO regulations (id, region_code, fish_species, min_size_cm, max_catch_per_day, season_start, season_end, restrictions)
VALUES
    ('ua-pike', 'UA', 'Щука', 35.0, 3, NULL, NULL, 'Заборона нересту: 15.03 - 30.04'),
    ('ua-zander', 'UA', 'Судак', 38.0, 3, NULL, NULL, 'Заборона нересту: 01.04 - 15.05'),
    ('ua-carp', 'UA', 'Короп', 30.0, 5, NULL, NULL, 'Обмеження в деяких водоймах'),
    ('ua-catfish', 'UA', 'Сом', 60.0, 1, NULL, NULL, 'Заборона нересту: 15.05 - 30.06'),
    ('ua-bream', 'UA', 'Лящ', 25.0, 5, NULL, NULL, 'Немає особливих обмежень'),
    ('ua-crucian', 'UA', 'Карась', 15.0, NULL, NULL, NULL, 'Без обмежень'),
    ('ua-perch', 'UA', 'Окунь', 12.0, 10, NULL, NULL, 'Без обмежень'),
    ('ua-roach', 'UA', 'Плітка', 10.0, NULL, NULL, NULL, 'Без обмежень');

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_fish_season ON fish_species(best_season);
CREATE INDEX IF NOT EXISTS idx_water_bodies_country ON water_bodies(country_code);
CREATE INDEX IF NOT EXISTS idx_water_bodies_type ON water_bodies(water_type);
CREATE INDEX IF NOT EXISTS idx_regulations_region ON regulations(region_code);
CREATE INDEX IF NOT EXISTS idx_regulations_species ON regulations(fish_species);
