//! ML Engine for Fishing Forecast Prediction
//!
//! Implements a Gradient Boosting model for predicting fish bite probability
//! based on historical catch data and environmental features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Feature set for ML prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FishingFeatures {
    /// Temperature in Celsius
    pub temperature_c: f64,
    /// Atmospheric pressure in hPa
    pub pressure_hpa: f64,
    /// Wind speed in m/s
    pub wind_speed_ms: f64,
    /// Wind direction in degrees (0-360)
    pub wind_direction_deg: Option<f64>,
    /// Precipitation in mm
    pub precipitation_mm: Option<f64>,
    /// Hour of day (0-23)
    pub hour: u32,
    /// Day of year (1-366)
    pub day_of_year: u32,
    /// Moon phase (0.0-1.0, new moon to full moon)
    pub moon_phase: f64,
    /// Moon illumination (0.0-1.0)
    pub moon_illumination: f64,
    /// Latitude for seasonal adjustments
    pub latitude: f64,
    /// Season factor (derived from day_of_year)
    pub season_factor: f64,
    /// Time of day category (0=night, 1=morning, 2=day, 3=evening)
    pub time_category: u32,
    /// Cloud cover (0-1)
    pub cloud_cover: Option<f64>,
    /// Humidity percentage (0-100)
    pub humidity: Option<f64>,
}

/// Historical catch record with features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    pub features: FishingFeatures,
    pub bite_intensity: f64, // 0.0 to 1.0
    pub success_rate: f64, // 0.0 to 1.0
}

/// Model weights for gradient boosting
#[derive(Debug, Clone)]
struct ModelWeights {
    /// Temperature coefficient
    temp_weight: f64,
    temp_threshold: f64,
    /// Pressure coefficient
    pressure_weight: f64,
    pressure_optimal: f64,
    /// Wind coefficient
    wind_weight: f64,
    wind_optimal: f64,
    /// Time of day coefficient
    morning_weight: f64,
    evening_weight: f64,
    /// Moon coefficient
    moon_weight: f64,
    moon_phase_optimal: f64,
    /// Season coefficient
    season_weight: f64,
    /// Precipitation penalty
    rain_penalty: f64,
    /// Cloud cover penalty
    cloud_penalty: f64,
    /// Bias term
    bias: f64,
}

/// Gradient Boosting Model for Fishing Forecast
#[derive(Debug, Clone)]
pub struct GradientBoostingModel {
    weights: ModelWeights,
    n_iterations: usize,
    learning_rate: f64,
    training_samples: Vec<TrainingSample>,
}

impl Default for GradientBoostingModel {
    fn default() -> Self {
        Self::new()
    }
}

impl GradientBoostingModel {
    /// Create a new model with default weights
    pub fn new() -> Self {
        Self {
            weights: ModelWeights {
                temp_weight: 0.25,
                temp_threshold: 18.0,
                pressure_weight: 0.20,
                pressure_optimal: 1013.0,
                wind_weight: 0.10,
                wind_optimal: 4.0,
                morning_weight: 0.15,
                evening_weight: 0.12,
                moon_weight: 0.08,
                moon_phase_optimal: 0.5,
                season_weight: 0.05,
                rain_penalty: 0.15,
                cloud_penalty: 0.05,
                bias: 0.35,
            },
            n_iterations: 100,
            learning_rate: 0.1,
            training_samples: Vec::new(),
        }
    }

    /// Create model with custom learning rate
    pub fn with_learning_rate(learning_rate: f64) -> Self {
        let mut model = Self::new();
        model.learning_rate = learning_rate;
        model
    }

    /// Calculate seasonal factor based on latitude and day of year
    fn calculate_seasonal_factor(day_of_year: u32, latitude: f64) -> f64 {
        // Approximate seasonal effect on fishing
        let northern_hemisphere = latitude >= 0.0;
        let day = if northern_hemisphere { day_of_year } else { (day_of_year + 180) % 365 };
        
        // Peak fishing seasons: spring (90-150) and fall (270-330)
        let spring_peak = 120.0; // May 1 approx
        let fall_peak = 300.0; // Oct 27 approx
        
        let spring_dist = ((day as f64 - spring_peak) / 365.0 * std::f64::consts::TAU).cos();
        let fall_dist = ((day as f64 - fall_peak) / 365.0 * std::f64::consts::TAU).cos();
        
        (spring_dist + fall_dist + 1.0) / 3.0 // Normalize to 0-1
    }

    /// Get time category from hour
    fn time_category(hour: u32) -> u32 {
        match hour {
            0..=5 => 0,      // Night
            6..=11 => 1,     // Morning
            12..=17 => 2,    // Day
            _ => 3,          // Evening
        }
    }

    /// Normalize moon phase to optimal distance
    fn moon_distance(phase: f64) -> f64 {
        // Optimal around new moon (0.0) and full moon (0.5) and new moon again (1.0)
        let dist_to_new = phase.abs();
        let dist_to_full = (phase - 0.5).abs();
        dist_to_new.min(dist_to_full).min(1.0 - dist_to_new)
    }

    /// Predict bite probability from features
    pub fn predict(&self, features: &FishingFeatures) -> f64 {
        // Temperature score - optimal around temp_threshold
        let temp_score = 1.0 - ((features.temperature_c - self.weights.temp_threshold) / 20.0).abs().clamp(0.0, 1.0);
        
        // Pressure score - optimal near sea level pressure
        let pressure_diff = (features.pressure_hpa - self.weights.pressure_optimal).abs();
        let pressure_score = 1.0 - (pressure_diff / 50.0).clamp(0.0, 1.0);
        
        // Wind score - light to moderate wind is best
        let wind_diff = (features.wind_speed_ms - self.weights.wind_optimal).abs();
        let wind_score = 1.0 - (wind_diff / 10.0).clamp(0.0, 1.0);
        
        // Time of day score
        let time_score = match features.time_category {
            1 => self.weights.morning_weight,   // Morning
            3 => self.weights.evening_weight,   // Evening
            2 => 0.05,                          // Midday
            _ => 0.02,                          // Night
        };
        
        // Moon score - prefers new/full moon
        let moon_dist = Self::moon_distance(features.moon_phase);
        let moon_score = self.weights.moon_weight * (1.0 - moon_dist);
        
        // Season score
        let season_score = self.weights.season_weight * features.season_factor;
        
        // Precipitation penalty
        let rain_penalty = if features.precipitation_mm.unwrap_or(0.0) > 0.5 {
            self.weights.rain_penalty * (features.precipitation_mm.unwrap_or(0.0) / 10.0).clamp(0.0, 1.0)
        } else {
            0.0
        };
        
        // Cloud cover penalty
        let cloud_penalty = features.cloud_cover.unwrap_or(0.0) * self.weights.cloud_penalty;
        
        // Calculate probability with sigmoid activation
        let z = self.weights.bias 
            + self.weights.temp_weight * temp_score
            + pressure_score * self.weights.pressure_weight
            + wind_score * self.weights.wind_weight
            + time_score
            + moon_score
            + season_score
            - rain_penalty
            - cloud_penalty;
        
        // Sigmoid activation
        1.0 / (1.0 + (-z * 3.0).exp())
    }

    /// Add training sample
    pub fn add_sample(&mut self, sample: TrainingSample) {
        self.training_samples.push(sample);
    }

    /// Add multiple training samples
    pub fn add_samples(&mut self, samples: Vec<TrainingSample>) {
        self.training_samples.extend(samples);
    }

    /// Train the model using gradient boosting (simplified)
    pub fn train(&mut self) {
        if self.training_samples.is_empty() {
            tracing::warn!("No training samples available");
            return;
        }

        // Simplified gradient boosting: iterate and adjust weights
        for _ in 0..self.n_iterations {
            let mut gradient_sum = 0.0;
            let mut weight_adjustments = HashMap::new();
            
            for sample in &self.training_samples {
                let prediction = self.predict(&sample.features);
                let error = sample.bite_intensity - prediction;
                gradient_sum += error;
                
                // Calculate gradients for each feature
                // Temperature gradient
                let temp_grad = error * (sample.features.temperature_c - self.weights.temp_threshold);
                *weight_adjustments.entry("temp".to_string()).or_insert(0.0) += temp_grad;
                
                // Pressure gradient
                let pressure_grad = error * (sample.features.pressure_hpa - self.weights.pressure_optimal);
                *weight_adjustments.entry("pressure".to_string()).or_insert(0.0) += pressure_grad;
                
                // Wind gradient
                let wind_grad = error * (sample.features.wind_speed_ms - self.weights.wind_optimal);
                *weight_adjustments.entry("wind".to_string()).or_insert(0.0) += wind_grad;
            }
            
            // Apply weight adjustments
            let avg_gradient = gradient_sum / self.training_samples.len() as f64;
            self.weights.bias += self.learning_rate * avg_gradient;
            
            if let Some(&temp_adj) = weight_adjustments.get("temp") {
                self.weights.temp_weight += self.learning_rate * temp_adj / self.training_samples.len() as f64;
                self.weights.temp_weight = self.weights.temp_weight.clamp(0.0, 0.5);
            }
            if let Some(&pressure_adj) = weight_adjustments.get("pressure") {
                self.weights.pressure_weight += self.learning_rate * pressure_adj / self.training_samples.len() as f64;
                self.weights.pressure_weight = self.weights.pressure_weight.clamp(0.0, 0.5);
            }
            if let Some(&wind_adj) = weight_adjustments.get("wind") {
                self.weights.wind_weight += self.learning_rate * wind_adj / self.training_samples.len() as f64;
                self.weights.wind_weight = self.weights.wind_weight.clamp(0.0, 0.3);
            }
        }
        
        tracing::info!("Model trained on {} samples", self.training_samples.len());
    }

    /// Get model statistics
    pub fn get_stats(&self) -> ModelStats {
        let n_samples = self.training_samples.len();
        let avg_prediction = if n_samples > 0 {
            self.training_samples.iter()
                .map(|s| self.predict(&s.features))
                .sum::<f64>() / n_samples as f64
        } else {
            0.0
        };
        
        ModelStats {
            n_samples,
            n_iterations: self.n_iterations,
            learning_rate: self.learning_rate,
            avg_prediction,
        }
    }
}

/// Model statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelStats {
    pub n_samples: usize,
    pub n_iterations: usize,
    pub learning_rate: f64,
    pub avg_prediction: f64,
}

/// Feature importance scores
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureImportance {
    pub temperature: f64,
    pub pressure: f64,
    pub wind: f64,
    pub time_of_day: f64,
    pub moon_phase: f64,
    pub season: f64,
    pub precipitation: f64,
    pub cloud_cover: f64,
}

impl FeatureImportance {
    /// Calculate from model weights
    pub fn from_model(model: &GradientBoostingModel) -> Self {
        let total = model.weights.temp_weight 
            + model.weights.pressure_weight 
            + model.weights.wind_weight 
            + model.weights.morning_weight 
            + model.weights.evening_weight
            + model.weights.moon_weight
            + model.weights.season_weight
            + model.weights.rain_penalty
            + model.weights.cloud_penalty
            + model.weights.bias;
        
        Self {
            temperature: model.weights.temp_weight / total,
            pressure: model.weights.pressure_weight / total,
            wind: model.weights.wind_weight / total,
            time_of_day: (model.weights.morning_weight + model.weights.evening_weight) / total,
            moon_phase: model.weights.moon_weight / total,
            season: model.weights.season_weight / total,
            precipitation: model.weights.rain_penalty / total,
            cloud_cover: model.weights.cloud_penalty / total,
        }
    }
}

/// Shared model registry for concurrent access
#[derive(Clone)]
pub struct ModelRegistry {
    model: Arc<RwLock<GradientBoostingModel>>,
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            model: Arc::new(RwLock::new(GradientBoostingModel::new())),
        }
    }

    /// Get model reference for reading
    pub async fn get(&self) -> tokio::sync::RwLockReadGuard<'_, GradientBoostingModel> {
        self.model.read().await
    }

    /// Get mutable reference for training
    pub async fn get_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, GradientBoostingModel> {
        self.model.write().await
    }

    /// Replace the model
    pub async fn set(&self, model: GradientBoostingModel) {
        let mut guard = self.model.write().await;
        *guard = model;
    }

    /// Predict with shared model
    pub async fn predict(&self, features: &FishingFeatures) -> f64 {
        self.model.read().await.predict(features)
    }
}

/// Prediction result with explanation
#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionResult {
    pub probability: f64,
    pub confidence: f64,
    pub factors: Vec<FactorScore>,
    pub best_time: String,
    pub recommendation: PredictionRecommendation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactorScore {
    pub name: String,
    pub score: f64,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictionRecommendation {
    Excellent,
    Good,
    Moderate,
    Poor,
    Avoid,
}

impl From<f64> for PredictionRecommendation {
    fn from(prob: f64) -> Self {
        match prob {
            p if p >= 0.8 => Self::Excellent,
            p if p >= 0.6 => Self::Good,
            p if p >= 0.4 => Self::Moderate,
            p if p >= 0.2 => Self::Poor,
            _ => Self::Avoid,
        }
    }
}

/// Generate detailed prediction with explanation
impl GradientBoostingModel {
    pub fn predict_detailed(&self, features: &FishingFeatures) -> PredictionResult {
        let probability = self.predict(features);
        
        // Calculate individual factor scores
        let temp_score = 1.0 - ((features.temperature_c - self.weights.temp_threshold) / 20.0).abs().clamp(0.0, 1.0);
        let pressure_score = 1.0 - ((features.pressure_hpa - self.weights.pressure_optimal) / 50.0).abs().clamp(0.0, 1.0);
        let wind_score = 1.0 - ((features.wind_speed_ms - self.weights.wind_optimal) / 10.0).abs().clamp(0.0, 1.0);
        let moon_dist = Self::moon_distance(features.moon_phase);
        let moon_score = 1.0 - moon_dist;
        
        let time_score = match features.time_category {
            1 => self.weights.morning_weight / 0.15,
            3 => self.weights.evening_weight / 0.12,
            2 => 0.3,
            _ => 0.1,
        };
        
        let rain_penalty = if features.precipitation_mm.unwrap_or(0.0) > 0.5 {
            self.weights.rain_penalty * 2.0
        } else {
            0.0
        };
        
        let factors = vec![
            FactorScore {
                name: "Temperature".to_string(),
                score: temp_score,
                impact: if temp_score > 0.7 { "Excellent" } else if temp_score > 0.4 { "Good" } else { "Poor" }.to_string(),
            },
            FactorScore {
                name: "Pressure".to_string(),
                score: pressure_score,
                impact: if pressure_score > 0.7 { "Stable" } else if pressure_score > 0.4 { "Moderate" } else { "Unstable" }.to_string(),
            },
            FactorScore {
                name: "Wind".to_string(),
                score: wind_score,
                impact: if wind_score > 0.7 { "Light breeze" } else if wind_score > 0.4 { "Moderate" } else { "Strong" }.to_string(),
            },
            FactorScore {
                name: "Moon Phase".to_string(),
                score: moon_score,
                impact: if moon_dist < 0.1 { "New/Full Moon" } else if moon_dist < 0.3 { "Near peak" } else { "Off peak" }.to_string(),
            },
            FactorScore {
                name: "Time of Day".to_string(),
                score: time_score.clamp(0.0, 1.0),
                impact: match features.time_category {
                    1 => "Morning (dawn)".to_string(),
                    3 => "Evening (dusk)".to_string(),
                    2 => "Midday".to_string(),
                    _ => "Night".to_string(),
                },
            },
            FactorScore {
                name: "Precipitation".to_string(),
                score: 1.0 - rain_penalty,
                impact: if rain_penalty < 0.05 { "Dry" } else if rain_penalty < 0.15 { "Light rain" } else { "Heavy rain" }.to_string(),
            },
        ];
        
        // Determine best time based on features
        let best_time = if features.hour >= 5 && features.hour <= 9 {
            "Now - Morning bite is active".to_string()
        } else if features.hour >= 17 && features.hour <= 21 {
            "Now - Evening bite is active".to_string()
        } else if features.hour >= 10 && features.hour <= 15 {
            "Best in 1-2 hours".to_string()
        } else {
            "Early morning or late evening recommended".to_string()
        };
        
        PredictionResult {
            probability,
            confidence: 0.5 + (self.training_samples.len() as f64 / 1000.0).clamp(0.0, 0.4),
            factors,
            best_time,
            recommendation: probability.into(),
        }
    }
}

/// Create features from weather and time data
#[inline]
pub fn create_features(
    temperature_c: f64,
    pressure_hpa: f64,
    wind_speed_ms: f64,
    wind_direction_deg: Option<f64>,
    precipitation_mm: Option<f64>,
    hour: u32,
    day_of_year: u32,
    moon_phase: f64,
    latitude: f64,
    cloud_cover: Option<f64>,
    humidity: Option<f64>,
) -> FishingFeatures {
    FishingFeatures {
        temperature_c,
        pressure_hpa,
        wind_speed_ms,
        wind_direction_deg,
        precipitation_mm,
        hour,
        day_of_year,
        moon_phase,
        moon_illumination: (moon_phase * std::f64::consts::PI).sin().abs(),
        latitude,
        season_factor: GradientBoostingModel::calculate_seasonal_factor(day_of_year, latitude),
        time_category: GradientBoostingModel::time_category(hour),
        cloud_cover,
        humidity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction_basic() {
        let model = GradientBoostingModel::new();
        
        // Good conditions for fishing
        let features = FishingFeatures {
            temperature_c: 18.0,
            pressure_hpa: 1013.0,
            wind_speed_ms: 4.0,
            wind_direction_deg: Some(180.0),
            precipitation_mm: Some(0.0),
            hour: 7,
            day_of_year: 120,
            moon_phase: 0.5,
            moon_illumination: 1.0,
            latitude: 52.0,
            season_factor: 0.8,
            time_category: 1,
            cloud_cover: Some(0.3),
            humidity: Some(60.0),
        };
        
        let prob = model.predict(&features);
        assert!(prob > 0.5, "Good conditions should have high probability: {}", prob);
    }

    #[test]
    fn test_prediction_bad_conditions() {
        let model = GradientBoostingModel::new();
        
        // Poor conditions for fishing
        let features = FishingFeatures {
            temperature_c: 5.0,
            pressure_hpa: 980.0,
            wind_speed_ms: 15.0,
            wind_direction_deg: Some(270.0),
            precipitation_mm: Some(10.0),
            hour: 14,
            day_of_year: 30,
            moon_phase: 0.25,
            moon_illumination: 0.5,
            latitude: 52.0,
            season_factor: 0.3,
            time_category: 2,
            cloud_cover: Some(0.9),
            humidity: Some(90.0),
        };
        
        let prob = model.predict(&features);
        assert!(prob < 0.5, "Bad conditions should have low probability: {}", prob);
    }

    #[test]
    fn test_create_features() {
        let features = create_features(
            20.0, 1015.0, 3.0, Some(90.0), None,
            8, 180, 0.3, 50.0, Some(0.5), Some(70.0)
        );
        
        assert_eq!(features.temperature_c, 20.0);
        assert_eq!(features.hour, 8);
        assert_eq!(features.time_category, 1);
        assert!(features.season_factor > 0.0);
    }
}
