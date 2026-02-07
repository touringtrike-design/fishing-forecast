INSERT INTO countries (code, name_en, name_local, region, currency, supported)
VALUES
    ('UA', 'Ukraine', 'Україна', 'Europe', 'UAH', TRUE),
    ('PL', 'Poland', 'Polska', 'Europe', 'PLN', TRUE),
    ('EU', 'European Union', 'European Union', 'Europe', 'EUR', TRUE)
ON CONFLICT (code) DO NOTHING;

INSERT INTO languages (code, name_en, name_local, active)
VALUES
    ('uk', 'Ukrainian', 'Українська', TRUE),
    ('en', 'English', 'English', TRUE),
    ('pl', 'Polish', 'Polski', TRUE),
    ('de', 'German', 'Deutsch', TRUE),
    ('fr', 'French', 'Français', TRUE)
ON CONFLICT (code) DO NOTHING;

WITH species AS (
    INSERT INTO fish_species (scientific_name, family, habitat)
    VALUES
        ('Cyprinus carpio', 'Cyprinidae', 'Freshwater'),
        ('Esox lucius', 'Esocidae', 'Freshwater'),
        ('Perca fluviatilis', 'Percidae', 'Freshwater'),
        ('Silurus glanis', 'Siluridae', 'Freshwater')
    ON CONFLICT DO NOTHING
    RETURNING id, scientific_name
)
INSERT INTO fish_names (fish_id, language, common_name)
SELECT s.id, l.language, l.common_name
FROM species s
JOIN (VALUES
    ('Cyprinus carpio', 'uk', 'Короп'),
    ('Cyprinus carpio', 'en', 'Carp'),
    ('Cyprinus carpio', 'pl', 'Karp'),
    ('Esox lucius', 'uk', 'Щука'),
    ('Esox lucius', 'en', 'Pike'),
    ('Esox lucius', 'pl', 'Szczupak'),
    ('Perca fluviatilis', 'uk', 'Окунь'),
    ('Perca fluviatilis', 'en', 'Perch'),
    ('Perca fluviatilis', 'pl', 'Okoń'),
    ('Silurus glanis', 'uk', 'Сом'),
    ('Silurus glanis', 'en', 'Catfish'),
    ('Silurus glanis', 'pl', 'Sum')
) AS l(scientific_name, language, common_name)
    ON s.scientific_name = l.scientific_name
ON CONFLICT DO NOTHING;

INSERT INTO fish_regions (fish_id, country_code, abundance, notes)
SELECT fs.id, r.country_code, r.abundance, r.notes
FROM fish_species fs
JOIN (VALUES
    ('Cyprinus carpio', 'UA', 'Common', NULL),
    ('Cyprinus carpio', 'PL', 'Common', NULL),
    ('Esox lucius', 'UA', 'Common', NULL),
    ('Esox lucius', 'PL', 'Common', NULL),
    ('Perca fluviatilis', 'UA', 'Common', NULL),
    ('Perca fluviatilis', 'PL', 'Common', NULL),
    ('Silurus glanis', 'UA', 'Common', NULL),
    ('Silurus glanis', 'PL', 'Common', NULL)
) AS r(scientific_name, country_code, abundance, notes)
    ON fs.scientific_name = r.scientific_name
ON CONFLICT DO NOTHING;

INSERT INTO fishing_regulations (
    country_code,
    region,
    fish_species_id,
    license_required,
    license_cost_local,
    license_url,
    min_size_cm,
    max_size_cm,
    daily_limit,
    closed_season_start,
    closed_season_end,
    protected,
    notes
)
SELECT
    r.country_code,
    r.region,
    fs.id,
    r.license_required,
    r.license_cost_local,
    r.license_url,
    r.min_size_cm,
    r.max_size_cm,
    r.daily_limit,
    r.closed_season_start,
    r.closed_season_end,
    r.protected,
    r.notes
FROM fish_species fs
JOIN (VALUES
    ('Cyprinus carpio', 'UA', NULL, TRUE, 'UAH 100/day', NULL, 25.0, NULL, 5, DATE '2026-04-01', DATE '2026-06-10', FALSE, 'Basic national rules'),
    ('Esox lucius', 'UA', NULL, TRUE, 'UAH 100/day', NULL, 35.0, NULL, 3, DATE '2026-02-15', DATE '2026-05-15', FALSE, 'Basic national rules'),
    ('Cyprinus carpio', 'PL', NULL, TRUE, 'PLN 30/day', NULL, 30.0, NULL, 3, DATE '2026-03-01', DATE '2026-05-31', FALSE, 'Basic national rules'),
    ('Esox lucius', 'PL', NULL, TRUE, 'PLN 30/day', NULL, 50.0, NULL, 2, DATE '2026-01-01', DATE '2026-04-30', FALSE, 'Basic national rules')
) AS r(scientific_name, country_code, region, license_required, license_cost_local, license_url, min_size_cm, max_size_cm, daily_limit, closed_season_start, closed_season_end, protected, notes)
    ON fs.scientific_name = r.scientific_name
ON CONFLICT DO NOTHING;

INSERT INTO prohibited_gear (country_code, region, gear_type, description, exceptions)
VALUES
    ('UA', NULL, 'Explosives', 'Explosive devices are prohibited', NULL),
    ('UA', NULL, 'Electric', 'Electric fishing devices are prohibited', NULL),
    ('PL', NULL, 'Nets', 'Use of nets for recreational fishing is prohibited', NULL)
ON CONFLICT DO NOTHING;
