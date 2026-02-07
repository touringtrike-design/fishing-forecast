## 🌦️ API погоди (безкоштовні)

### 1. **Open-Meteo** ⭐ НАЙКРАЩЕ для вас

- **URL**: [https://open-meteo.com/](https://open-meteo.com/)
- **Ліміти**: Безлімітно для некомерційного використання
- **Реєстрація**: НЕ потрібна
- **Дані**:
    - Температура повітря (поточна, прогноз 16 днів)
    - Атмосферний тиск
    - Вологість
    - Швидкість та напрямок вітру
    - Хмарність, опади
    - UV індекс
    - Видимість
- **Особливості**:
    - ✅ Історичні дані з 1940 року
    - ✅ Погодинний прогноз
    - ✅ Немає API ключа
    - ✅ CORS enabled
    - ✅ Швидкий (edge network)

// Cargo.toml
// [dependencies]
// reqwest = { version = "0.11", features = ["json"] }
// serde = { version = "1.0", features = ["derive"] }

use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    hourly: HourlyData,
    daily: DailyData,
}

#[derive(Debug, Deserialize)]
struct HourlyData {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    relative_humidity_2m: Vec<i32>,
    pressure_msl: Vec<f64>,
    wind_speed_10m: Vec<f64>,
    wind_direction_10m: Vec<i32>,
    cloud_cover: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct DailyData {
    time: Vec<String>,
    sunrise: Vec<String>,
    sunset: Vec<String>,
    precipitation_sum: Vec<f64>,
}

async fn get_weather(lat: f64, lon: f64) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?\
         latitude={}&longitude={}&\
         hourly=temperature_2m,relative_humidity_2m,pressure_msl,\
         wind_speed_10m,wind_direction_10m,cloud_cover&\
         daily=sunrise,sunset,precipitation_sum&\
         timezone=auto",
        lat, lon
    );
    
    let response = reqwest::get(&url)
        .await?
        .json::<WeatherResponse>()
        .await?;
    
    Ok(response)
}

### 2. **OpenWeatherMap** (обмежений безкоштовний)

- **URL**: [https://openweathermap.org/api](https://openweathermap.org/api)
- **Ліміти**: 1000 викликів/день, 60 викликів/хв
- **Реєстрація**: Потрібна (безкоштовна)
- **Дані**:
    - Поточна погода
    - Прогноз на 5 днів (3-годинні інтервали)
    - Історичні дані (платно)
- **Мінуси**:
    - ❌ Обмежені ліміти
    - ❌ Платний прогноз >5 днів

---

### 3. **WeatherAPI** (generous free tier)

- **URL**: [https://www.weatherapi.com/](https://www.weatherapi.com/)
- **Ліміти**: 1,000,000 викликів/місяць безкоштовно
- **Реєстрація**: Потрібна
- **Дані**:
    - Поточна погода
    - Прогноз на 3 дні
    - Астрономічні дані (схід/захід сонця, фази місяця!)
    - Історичні дані (7 днів назад)

**Приклад для фаз місяця:**
#[derive(Debug, Deserialize)]
struct Astronomy {
    moon_phase: String,        // "New Moon", "Full Moon", etc.
    moon_illumination: String, // "0", "50", "100"
}

async fn get_moon_phase(lat: f64, lon: f64, date: &str) -> Result<Astronomy, reqwest::Error> {
    let url = format!(
        "https://api.weatherapi.com/v1/astronomy.json?\
         key=YOUR_API_KEY&q={},{}&dt={}",
        lat, lon, date
    );
    
    let response = reqwest::get(&url)
        .await?
        .json::<AstronomyResponse>()
        .await?;
    
    Ok(response.astronomy.astro)
}
### 4. **Brightsky** (тільки Німеччина, але open source)

- **URL**: [https://brightsky.dev/](https://brightsky.dev/)
- **Ліміти**: Безлімітно
- **Реєстрація**: НЕ потрібна
- **Особливості**:
    - ✅ Повністю open source
    - ✅ Можна розгорнути свій інстанс
    - ❌ Тільки Німеччина
## 🗺️ API водойм і геоданих

### 1. **Overpass API (OpenStreetMap)** ⭐ ОСНОВНЕ джерело

- **URL**: [https://overpass-api.de/](https://overpass-api.de/)
- **Ліміти**: Розумне використання (не більше 10K запитів/день)
- **Реєстрація**: НЕ потрібна
- **Дані**:
    - Озера, річки, ставки, водосховища
    - Координати водойм
    - Назви водойм
    - Додаткові теги (розмір, глибина якщо є)

**Приклад запиту водойм:**
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct OverpassResponse {
    elements: Vec<WaterBody>,
}

#[derive(Debug, Deserialize)]
struct WaterBody {
    #[serde(rename = "type")]
    element_type: String,
    id: i64,
    lat: Option<f64>,
    lon: Option<f64>,
    tags: Option<WaterTags>,
    center: Option<Center>,
}

#[derive(Debug, Deserialize)]
struct WaterTags {
    name: Option<String>,
    water: Option<String>, // "lake", "river", "pond", "reservoir"
    #[serde(rename = "natural")]
    natural_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Center {
    lat: f64,
    lon: f64,
}

async fn get_water_bodies_nearby(lat: f64, lon: f64, radius_m: u32) -> Result<Vec<WaterBody>, reqwest::Error> {
    // Overpass QL запит
    let query = format!(
        r#"
        [out:json];
        (
          way["natural"="water"](around:{},{},{});
          relation["natural"="water"](around:{},{},{});
        );
        out center;
        "#,
        radius_m, lat, lon,
        radius_m, lat, lon
    );
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()
        .await?
        .json::<OverpassResponse>()
        .await?;
    
    Ok(response.elements)
}

// Приклад: отримати всі озера в радіусі 10 км
async fn get_lakes_near_kyiv() {
    let water_bodies = get_water_bodies_nearby(50.4501, 30.5234, 10000).await.unwrap();
    
    for body in water_bodies {
        if let Some(tags) = &body.tags {
            if tags.water.as_deref() == Some("lake") {
                println!("Озеро: {:?}", tags.name);
            }
        }
    }
}

Розширений запит з більше даними:
// Запит з додатковими параметрами
let detailed_query = r#"
[out:json];
(
  way["natural"="water"]["water"~"lake|pond|reservoir"](around:5000,50.4501,30.5234);
  relation["natural"="water"]["water"~"lake|pond|reservoir"](around:5000,50.4501,30.5234);
);
out body;
>;
out skel qt;
"#;
### 2. **Nominatim (OpenStreetMap Geocoding)** ⭐

- **URL**: [https://nominatim.openstreetmap.org/](https://nominatim.openstreetmap.org/)
- **Ліміти**: 1 запит/секунду
- **Реєстрація**: НЕ потрібна (рекомендовано вказати User-Agent)
- **Дані**:
    - Reverse geocoding (координати → адреса)
    - Geocoding (адреса → координати)
    - Пошук водойм за назвою

**Приклад:**
#[derive(Debug, Deserialize)]
struct NominatimResult {
    place_id: i64,
    lat: String,
    lon: String,
    display_name: String,
    #[serde(rename = "type")]
    place_type: String,
}

async fn search_water_body(name: &str) -> Result<Vec<NominatimResult>, reqwest::Error> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?\
         q={}&format=json&\
         featuretype=natural&\
         limit=10",
        urlencoding::encode(name)
    );
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "FishingForecastApp/1.0")
        .send()
        .await?
        .json::<Vec<NominatimResult>>()
        .await?;
    
    Ok(response)
}

// Приклад: знайти Дніпро
let results = search_water_body("Dnipro river Ukraine").await.unwrap();
### 3. **GeoNames** (географічні дані)

- **URL**: [https://www.geonames.org/](https://www.geonames.org/)
- **Ліміти**: 20,000 credits/день безкоштовно (1 запит = 1 credit)
- **Реєстрація**: Потрібна (безкоштовна)
- **Дані**:
    - Озера, річки (з бази даних)
    - Висота над рівнем моря
    - Часові пояси

---

### 4. **HydroSHEDS** (гідрологічні дані)

- **URL**: [https://www.hydrosheds.org/](https://www.hydrosheds.org/)
- **Ліміти**: Безкоштовно
- **Формат**: Завантаження GIS файлів
- **Дані**:
    - Річкові системи
    - Водозбори
    - Висотні дані

---

## 🐟 API про рибу та водойми

### 1. **FishBase API**

- **URL**: [https://fishbase.ropensci.org/](https://fishbase.ropensci.org/)
- **Ліміти**: Безлімітно
- **Реєстрація**: НЕ потрібна
- **Дані**:
    - 34,000+ видів риб
    - Середовище існування
    - Географічне поширення

**Приклад:**
#[derive(Debug, Deserialize)]
struct FishSpecies {
    #[serde(rename = "SpecCode")]
    spec_code: i32,
    #[serde(rename = "Genus")]
    genus: String,
    #[serde(rename = "Species")]
    species: String,
    #[serde(rename = "FBname")]
    common_name: Option<String>,
}

async fn search_fish(name: &str) -> Result<Vec<FishSpecies>, reqwest::Error> {
    let url = format!(
        "https://fishbase.ropensci.org/species?Genus={}",
        name
    );
    
    reqwest::get(&url)
        .await?
        .json::<Vec<FishSpecies>>()
        .await
}

// Приклад: знайти коропа
let carp = search_fish("Cyprinus").await.unwrap();
### 2. **iNaturalist API** (спостереження природи)

- **URL**: [https://api.inaturalist.org/v1/](https://api.inaturalist.org/v1/)
- **Ліміти**: 100 запитів/хв
- **Реєстрація**: НЕ потрібна
- **Дані**:
    - Спостереження риб у водоймах
    - Фото
    - Геолокація

**Приклад:**
async fn get_fish_observations(lat: f64, lon: f64, radius_km: f64) -> Result<ObservationsResponse, reqwest::Error> {
    let url = format!(
        "https://api.inaturalist.org/v1/observations?\
         taxon_id=47178&\
         lat={}&lng={}&radius={}&\
         per_page=50",
        lat, lon, radius_km
    );
    
    reqwest::get(&url)
        .await?
        .json::<ObservationsResponse>()
        .await
}
## 🌙 Астрономічні дані (фази місяця)

### 1. **Astronomy API**

- **URL**: [https://astronomyapi.com/](https://astronomyapi.com/)
- **Ліміти**: Безкоштовно (з обмеженнями)
- **Реєстрація**: Потрібна

### 2. **USNO Astronomical Applications** (найточніше)

- **URL**: [https://aa.usno.navy.mil/data/api](https://aa.usno.navy.mil/data/api)
- **Ліміти**: Розумне використання
- **Реєстрація**: НЕ потрібна
- **Дані**:
    - Фази місяця
    - Схід/захід сонця та місяця
    - Сутінки

**Приклад:**
#[derive(Debug, Deserialize)]
struct MoonPhase {
    phase: String,
    date: String,
}

async fn get_moon_phases(year: i32) -> Result<Vec<MoonPhase>, reqwest::Error> {
    let url = format!(
        "https://aa.usno.navy.mil/api/moon/phases/year?year={}",
        year
    );
    
    reqwest::get(&url)
        .await?
        .json::<MoonPhasesResponse>()
        .await
}
## 💧 API якості води (додатково)

### 1. **USGS Water Services** (США)

- **URL**: [https://waterservices.usgs.gov/](https://waterservices.usgs.gov/)
- **Ліміти**: Безлімітно
- **Реєстрація**: НЕ потрібна
- **Дані**:
    - Рівень води
    - Температура води
    - Швидкість течії

### 2. **EEA Water Quality** (Європа)

- **URL**: [https://www.eea.europa.eu/data-and-maps/data](https://www.eea.europa.eu/data-and-maps/data)
- **Формат**: Завантаження датасетів
## 🎣 Додаткові джерела (crowdsourcing)

### 1. **Fishbrain API** (неофіційний)

- Соціальна мережа рибалок
- Можна парсити публічні дані

### 2. **Reddit API** (r/fishing)

- **URL**: [https://www.reddit.com/dev/api](https://www.reddit.com/dev/api)
- Аналіз звітів рибалок

📦 Повний Rust стек для проекту

# Cargo.toml
[package]
name = "fishing-forecast"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "macros", "uuid", "chrono"] }

# Geo types
geo = "0.27"
geojson = "0.24"

# Date/time
chrono = { version = "0.4", features = ["serde"] }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Environment variables
dotenv = "0.15"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# UUID
uuid = { version = "1.0", features = ["v4", "serde"] }

🏗️ Архітектура сервісів

// src/services/mod.rs
pub mod weather;
pub mod water_bodies;
pub mod fish_data;
pub mod astronomy;

// src/services/weather.rs
use reqwest;
use serde::{Deserialize, Serialize};

pub struct WeatherService {
    client: reqwest::Client,
}

impl WeatherService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn get_forecast(&self, lat: f64, lon: f64) -> Result<WeatherData, anyhow::Error> {
        // Open-Meteo (primary)
        match self.fetch_open_meteo(lat, lon).await {
            Ok(data) => Ok(data),
            Err(_) => {
                // Fallback to WeatherAPI
                self.fetch_weather_api(lat, lon).await
            }
        }
    }
    
    async fn fetch_open_meteo(&self, lat: f64, lon: f64) -> Result<WeatherData, anyhow::Error> {
        // Implementation
        todo!()
    }
    
    async fn fetch_weather_api(&self, lat: f64, lon: f64) -> Result<WeatherData, anyhow::Error> {
        // Fallback implementation
        todo!()
    }
}

// src/services/water_bodies.rs
pub struct WaterBodiesService {
    client: reqwest::Client,
}

impl WaterBodiesService {
    pub async fn find_nearby(&self, lat: f64, lon: f64, radius_m: u32) -> Result<Vec<WaterBody>, anyhow::Error> {
        // Overpass API query
        todo!()
    }
    
    pub async fn search_by_name(&self, name: &str) -> Result<Vec<WaterBody>, anyhow::Error> {
        // Nominatim search
        todo!()
    }
}

## 🎯 Рекомендований комплект API

### Для MVP використовуйте:

1. **Open-Meteo** - погода (пріоритет #1)
    - ✅ Безлімітно безкоштовно
    - ✅ Немає реєстрації
    - ✅ Всі потрібні параметри
2. **Overpass API (OSM)** - водойми
    - ✅ Детальні дані
    - ✅ Безкоштовно
    - ✅ Можна кешувати
3. **WeatherAPI** - фази місяця
    - ✅ 1M запитів/місяць
    - ✅ Астрономічні дані
4. **Nominatim** - геокодінг
    - ✅ Безкоштовно
    - ✅ OSM дані
5. **FishBase** - інформація про рибу
    - ✅ Безлімітно
    - ✅ Велика база

### Резервні варіанти:

- **OpenWeatherMap** (якщо Open-Meteo недоступний)
- **GeoNames** (якщо Nominatim повільний)

📋 СПИСОК БЕЗКОШТОВНИХ API

### 1. **Погода: Open-Meteo** ⭐⭐⭐⭐⭐

yaml

````yaml
URL: https://api.open-meteo.com/v1/forecast
Ліміт: Безлімітно (некомерційне використання)
Реєстрація: Не потрібна
CORS: Підтримується ✅

Параметри:
  - temperature_2m (повітря)
  - pressure_msl (тиск на рівні моря)
  - relative_humidity_2m
  - wind_speed_10m
  - wind_direction_10m
  - cloud_cover
  - precipitation
  - visibility (обмежено)
  
Прогноз: 16 днів вперед
Історія: З 1940 року
Оновлення: Кожну годину
```

**Чому краще за OpenWeatherMap:**
- ✅ Безлімітно (OWM: 1000 викликів/день)
- ✅ Немає API ключа
- ✅ Більше історичних даних
- ✅ Швидший response

**Приклад запиту:**
```
https://api.open-meteo.com/v1/forecast?
  latitude=50.45&
  longitude=30.52&
  hourly=temperature_2m,pressure_msl,relative_humidity_2m,wind_speed_10m&
  daily=sunrise,sunset&
  timezone=Europe/Kiev&
  past_days=3
````

---

### 2. **Водойми: Overpass API (OSM)** ⭐⭐⭐⭐⭐

yaml

```yaml
URL: https://overpass-api.de/api/interpreter
Ліміт: ~10K запитів/день (reasonable use)
Реєстрація: Не потрібна
CORS: Підтримується ✅

Дані:
  - natural=water (озера, ставки)
  - waterway=river (річки)
  - landuse=reservoir (водосховища)
  - Координати, назви, площа
  - Додаткові теги (глибина, якщо є)
```

**Приклад запиту (водойми в радіусі 5км):**

overpassql

```overpassql
[out:json];
(
  way["natural"="water"](around:5000,50.45,30.52);
  relation["natural"="water"](around:5000,50.45,30.52);
);
out center;
```

**Важливо:** Кешувати результати! Не робити один запит на кожен рух карти.

---

### 3. **Геокодінг: Nominatim (OSM)** ⭐⭐⭐⭐

yaml

````yaml
URL: https://nominatim.openstreetmap.org/
Ліміт: 1 запит/секунду
Реєстрація: Не потрібна (але вказати User-Agent!)
CORS: Підтримується ✅

Функції:
  - Пошук за назвою → координати
  - Reverse (координати → адреса)
  - Пошук водойм за назвою
```

**Приклад:**
```
https://nominatim.openstreetmap.org/search?
  q=Дніпро+Київ&
  format=json&
  limit=5
````

---

### 4. **Фази місяця: Локальні обчислення** ⭐⭐⭐⭐⭐

**НЕ потрібен API!** Розраховується математично:

dart

```dart
// Flutter package
import 'package:moon_phase_calculator/moon_phase_calculator.dart';

double getMoonPhase(DateTime date) {
  const synodicMonth = 29.530588853;
  final newMoon2000 = DateTime(2000, 1, 6, 18, 14);
  
  final days = date.difference(newMoon2000).inDays;
  final phase = (days % synodicMonth) / synodicMonth;
  
  return phase;
}
```

**Переваги:**

- ✅ Безлімітно
- ✅ Працює офлайн
- ✅ Точність 99.9%
- ✅ Не треба інтернет

---

### 5. **Схід/захід сонця: SunCalc (локально)** ⭐⭐⭐⭐⭐

dart

```dart
import 'package:suncalc/suncalc.dart';

final times = SunCalc.getTimes(DateTime.now(), 50.45, 30.52);
print(times.sunrise);  // DateTime
print(times.sunset);   // DateTime
print(times.goldenHour); // Золота година

// Також доступно в Open-Meteo!
```

---

### 6. **Інформація про рибу: FishBase API** ⭐⭐⭐

yaml

````yaml
URL: https://fishbase.ropensci.org/
Ліміт: Безлімітно
Реєстрація: Не потрібна
CORS: Підтримується ✅

Дані:
  - 34,000+ видів риб
  - Наукові назви
  - Середовище існування
  - Географічне поширення
```

**Приклад:**
```
https://fishbase.ropensci.org/species?Genus=Cyprinus&Species=carpio
````
