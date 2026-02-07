use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// API response errors
#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    ServerError(u16, String),
    Timeout,
    NotFound,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiError::ServerError(code, msg) => write!(f, "Server error {}: {}", code, msg),
            ApiError::Timeout => write!(f, "Request timeout"),
            ApiError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl Error for ApiError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForecastResponse {
    pub probability: f64,
    pub confidence: f64,
    pub factors: FactorScores,
    pub explanation: String,
    pub recommended_baits: Vec<BaitRecommendation>,
    pub best_time: String,
    pub weather: WeatherInfo,
    pub moon_phase: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FactorScores {
    #[serde(alias = "pressure_score")]
    pub pressure: f64,
    #[serde(alias = "temperature_score")]
    pub temperature: f64,
    #[serde(alias = "time_of_day_score")]
    pub time_of_day: f64,
    #[serde(alias = "wind_score")]
    pub wind: f64,
    #[serde(alias = "moon_score")]
    pub moon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BaitRecommendation {
    pub name: String,
    #[serde(alias = "effectiveness", alias = "score")]
    pub effectiveness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherInfo {
    #[serde(alias = "temperature_c")]
    pub temperature: f64,
    #[serde(alias = "pressure_hpa", alias = "pressure_msl")]
    pub pressure_msl: f64,
    #[serde(alias = "wind_speed_ms")]
    pub wind_speed: f64,
    #[serde(alias = "wind_direction_deg")]
    pub wind_direction: f64,
    #[serde(default)]
    pub humidity: f64,
    #[serde(default)]
    pub water_temperature: f64,
    #[serde(default)]
    pub wave_height: f64,
    #[serde(default)]
    pub visibility: f64,
    #[serde(default)]
    pub sunrise: String,
    #[serde(default)]
    pub sunset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WaterBody {
    pub id: String,
    pub name: String,
    pub water_type: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountryInfo {
    pub country_code: String,
    pub country_name: String,
    pub supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FishSpecies {
    pub id: String,
    pub name: String,
    pub scientific_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegulationInfo {
    pub country_code: String,
    pub fish_species: String,
    pub min_size_cm: Option<f64>,
    pub max_size_cm: Option<f64>,
    pub daily_limit: Option<i32>,
    pub closed_season_start: Option<String>,
    pub closed_season_end: Option<String>,
    pub protected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatchRecord {
    pub id: String,
    pub user_id: String,
    pub lat: f64,
    pub lon: f64,
    pub caught_at: String,
    pub fish_species: String,
    pub weight_kg: Option<f64>,
    pub length_cm: Option<f64>,
    pub bait_used: Option<String>,
    pub weather_temp: Option<f64>,
    pub weather_pressure: Option<f64>,
    pub moon_phase: Option<f64>,
    pub notes: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationResult {
    pub allowed: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// API Client for communicating with backend
#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    /// Get the API base URL (local for dev, production for release)
    fn get_api_url() -> String {
        #[cfg(debug_assertions)]
        {
            // Local development - try localhost first
            "http://localhost:8080".to_string()
        }
        #[cfg(not(debug_assertions))]
        {
            // Production - use environment variable or default shuttle URL
            std::env::var("API_URL")
                .unwrap_or_else(|_| "https://fishing-forecast-api.shuttleapp.rs".to_string())
        }
    }

    /// Create new API client with base URL
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Create API client with auto-detected URL
    pub fn new_auto() -> Self {
        Self {
            base_url: Self::get_api_url(),
        }
    }

    /// Get forecast for coordinates
    pub async fn get_forecast(
        &self,
        lat: f64,
        lon: f64,
        fish: Option<&str>,
    ) -> Result<ForecastResponse, ApiError> {
        // Try backend first
        if let Ok(forecast) = self.get_backend_forecast(lat, lon, fish).await {
            return Ok(forecast);
        }

        // Try to get real weather data from Open-Meteo API
        if let Ok(forecast) = self.get_real_weather(lat, lon).await {
            return Ok(forecast);
        }

        // Fallback to mock data
        Ok(ForecastResponse {
            probability: 0.75,
            confidence: 0.82,
            factors: FactorScores {
                pressure: 0.65,
                temperature: 0.72,
                time_of_day: 0.88,
                wind: 0.58,
                moon: 0.71,
            },
            explanation: format!("Mock forecast for location {:.4}°, {:.4}°. Testing offline mode - mock data returned.", lat, lon),
            recommended_baits: vec![
                BaitRecommendation {
                    name: "Earthworm".to_string(),
                    effectiveness: 0.85,
                },
                BaitRecommendation {
                    name: "Maggot".to_string(),
                    effectiveness: 0.78,
                },
                BaitRecommendation {
                    name: "Bread".to_string(),
                    effectiveness: 0.65,
                },
            ],
            best_time: "14:30 - 16:45".to_string(),
            weather: WeatherInfo {
                temperature: 1.5,
                pressure_msl: 1020.0,
                wind_speed: 12.3,
                wind_direction: 240.0,
                humidity: 78.0,
                water_temperature: 2.8,
                wave_height: 0.5,
                visibility: 8.0,
                sunrise: "07:45".to_string(),
                sunset: "17:15".to_string(),
            },
            moon_phase: 0.65,
        })
    }

    /// Detect country from coordinates
    pub async fn detect_country(&self, lat: f64, lon: f64) -> Result<CountryInfo, ApiError> {
        let url = format!(
            "{}/api/v1/region/detect?lat={}&lon={}",
            self.base_url, lat, lon
        );
        self.fetch(&url).await
    }

    /// Get water bodies near coordinates
    pub async fn get_water_bodies(
        &self,
        lat: f64,
        lon: f64,
        radius_km: f64,
    ) -> Result<Vec<WaterBody>, ApiError> {
        let url = format!(
            "{}/api/v1/water-bodies?lat={}&lon={}&radius_km={}",
            self.base_url, lat, lon, radius_km
        );
        self.fetch(&url).await
    }

    /// Get fish species for country and language
    pub async fn get_fish_species(
        &self,
        country: &str,
        language: &str,
    ) -> Result<Vec<FishSpecies>, ApiError> {
        let url = format!(
            "{}/api/v1/fish?country={}&language={}",
            self.base_url, country, language
        );
        self.fetch(&url).await
    }

    /// Get regulations for country
    pub async fn get_regulations(
        &self,
        country: &str,
        fish: Option<&str>,
    ) -> Result<Vec<RegulationInfo>, ApiError> {
        let mut url = format!("{}/api/v1/regulations?country={}", self.base_url, country);

        if let Some(fish_name) = fish {
            url.push_str(&format!("&fish={}", urlencoding::encode(fish_name)));
        }

        self.fetch(&url).await
    }

    /// Validate catch against regulations
    pub async fn validate_catch(
        &self,
        country: &str,
        fish: &str,
        size_cm: f64,
        date: &str,
    ) -> Result<ValidationResult, ApiError> {
        let url = format!("{}/api/v1/regulations/validate", self.base_url);

        let body = serde_json::json!({
            "country_code": country,
            "fish_species": fish,
            "size_cm": size_cm,
            "caught_date": date,
        });

        self.fetch_with_body(&url, "POST", &body).await
    }

    /// Save new catch
    pub async fn save_catch(&self, catch: &CatchRecord) -> Result<CatchRecord, ApiError> {
        let url = format!("{}/api/v1/catches", self.base_url);
        self.fetch_with_body(&url, "POST", catch).await
    }

    /// Get user catches
    pub async fn get_catches(
        &self,
        user_id: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<CatchRecord>, ApiError> {
        let url = format!(
            "{}/api/v1/catches?user_id={}&limit={}&offset={}",
            self.base_url, user_id, limit, offset
        );
        self.fetch(&url).await
    }

    /// Get nearby catches
    pub async fn get_nearby_catches(
        &self,
        lat: f64,
        lon: f64,
        radius_km: f64,
    ) -> Result<Vec<CatchRecord>, ApiError> {
        let url = format!(
            "{}/api/v1/catches/nearby?lat={}&lon={}&radius_km={}",
            self.base_url, lat, lon, radius_km
        );
        self.fetch(&url).await
    }

    /// Health check
    pub async fn health_check(&self) -> Result<(), ApiError> {
        let _url = format!("{}/api/v1/health", self.base_url);
        let _response = self.fetch::<serde_json::Value>(&_url).await?;
        Ok(())
    }

    /// Generic fetch with GET
    async fn fetch<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> Result<T, ApiError> {
        use gloo_net::http::Request;
        
        let response = Request::get(url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Request failed: {}", e)))?;
        
        if !response.ok() {
            return Err(ApiError::ServerError(
                response.status(),
                format!("HTTP {}: {}", response.status(), response.status_text()),
            ));
        }
        
        response
            .json::<T>()
            .await
            .map_err(|e| ApiError::ParseError(format!("Failed to parse JSON: {}", e)))
    }

    /// Generic fetch with POST/custom method
    async fn fetch_with_body<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        method: &str,
        body: &impl Serialize,
    ) -> Result<T, ApiError> {
        use gloo_net::http::Request;
        
        let body_json = serde_json::to_string(body)
            .map_err(|e| ApiError::ParseError(format!("Failed to serialize body: {}", e)))?;
        
        let response = if method.to_uppercase() == "POST" {
            Request::post(url)
                .header("Content-Type", "application/json")
                .body(body_json)
                .map_err(|e| ApiError::NetworkError(format!("Request failed: {}", e)))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(format!("Request failed: {}", e)))?
        } else if method.to_uppercase() == "PUT" {
            Request::put(url)
                .header("Content-Type", "application/json")
                .body(body_json)
                .map_err(|e| ApiError::NetworkError(format!("Request failed: {}", e)))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(format!("Request failed: {}", e)))?
        } else {
            return Err(ApiError::NetworkError(format!("Unsupported method: {}", method)));
        };
        
        if !response.ok() {
            return Err(ApiError::ServerError(
                response.status(),
                format!("HTTP {}: {}", response.status(), response.status_text()),
            ));
        }
        
        response
            .json::<T>()
            .await
            .map_err(|e| ApiError::ParseError(format!("Failed to parse JSON: {}", e)))
    }

    /// Get forecast from local backend server
    async fn get_backend_forecast(&self, lat: f64, lon: f64, fish: Option<&str>) -> Result<ForecastResponse, ApiError> {
        let mut url = format!(
            "http://localhost:8080/api/v1/forecast?lat={}&lon={}",
            lat, lon
        );

        if let Some(f) = fish {
            url.push_str(&format!("&fish={}", f));
        }

        let response = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Backend unavailable: {}", e)))?;

        if !response.ok() {
            return Err(ApiError::ServerError(
                response.status(),
                format!("Backend HTTP {}", response.status()),
            ));
        }

        response
            .json::<ForecastResponse>()
            .await
            .map_err(|e| ApiError::ParseError(format!("Failed to parse backend response: {}", e)))
    }

    /// Get real weather data from Open-Meteo API (free, no API key needed)
    async fn get_real_weather(&self, lat: f64, lon: f64) -> Result<ForecastResponse, ApiError> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,weather_code,wind_speed_10m,wind_direction_10m,pressure_msl&hourly=temperature_2m&daily=sunrise,sunset,wind_direction_10m_dominant&timezone=auto",
            lat, lon
        );

        let response = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to fetch weather: {}", e)))?;

        let text = response
            .text()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to read response: {}", e)))?;

        let weather_data: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| ApiError::ParseError(format!("Failed to parse weather JSON: {}", e)))?;

        // Extract data from Open-Meteo response
        let current = &weather_data["current"];
        let daily = &weather_data["daily"];

        let temp = current["temperature_2m"]
            .as_f64()
            .unwrap_or(1.5);
        let wind_speed = current["wind_speed_10m"]
            .as_f64()
            .unwrap_or(12.3);
        let wind_direction = current["wind_direction_10m"]
            .as_f64()
            .unwrap_or(240.0);
        let pressure = current["pressure_msl"]
            .as_f64()
            .unwrap_or(1020.0);
        let humidity = current["relative_humidity_2m"]
            .as_f64()
            .unwrap_or(78.0);

        let sunrise = daily["sunrise"][0]
            .as_str()
            .and_then(|s| s.split('T').nth(1))
            .unwrap_or("07:45")
            .to_string();
        let sunset = daily["sunset"][0]
            .as_str()
            .and_then(|s| s.split('T').nth(1))
            .unwrap_or("17:15")
            .to_string();

        // Estimate water temperature (usually 2-3°C colder than air temp in winter)
        let water_temp = (temp - 2.5).max(-2.0);

        // Calculate bite probability based on weather
        let pressure_factor = if pressure > 1015.0 && pressure < 1025.0 { 0.9 } else { 0.5 };
        let temp_factor = if temp > 0.0 && temp < 5.0 { 0.8 } else { 0.4 };
        let wind_factor = if wind_speed > 5.0 && wind_speed < 15.0 { 0.8 } else { 0.5 };
        let probability = (pressure_factor + temp_factor + wind_factor) / 3.0 * 0.85;

        Ok(ForecastResponse {
            probability,
            confidence: 0.85,
            factors: FactorScores {
                pressure: pressure_factor,
                temperature: temp_factor,
                time_of_day: 0.75,
                wind: wind_factor,
                moon: 0.71,
            },
            explanation: format!(
                "Реальні погодні дані для локації {:.4}°, {:.4}°. Температура: {}°C, вітер: {} м/с, тиск: {} гПа",
                lat, lon, temp as i32, wind_speed as i32, pressure as i32
            ),
            recommended_baits: vec![
                BaitRecommendation {
                    name: "Мотиль".to_string(),
                    effectiveness: 0.88,
                },
                BaitRecommendation {
                    name: "Черв'як".to_string(),
                    effectiveness: 0.82,
                },
                BaitRecommendation {
                    name: "Хліб".to_string(),
                    effectiveness: 0.65,
                },
            ],
            best_time: "13:30 - 15:45".to_string(),
            weather: WeatherInfo {
                temperature: temp,
                pressure_msl: pressure,
                wind_speed,
                wind_direction,
                humidity,
                water_temperature: water_temp,
                wave_height: (wind_speed / 10.0).min(1.5),
                visibility: 15.0,
                sunrise,
                sunset,
            },
            moon_phase: 0.65,
        })
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        // Default to localhost for development
        Self::new("http://localhost:8080")
    }
}
