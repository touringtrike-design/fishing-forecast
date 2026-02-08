use anyhow::Context;
use chrono::{Datelike, Timelike};
use fishing_ml_engine::{
    create_features, FeatureImportance, ModelRegistry,
    PredictionResult, TrainingSample,
};
use fishing_shared::{
    types::{BaitRecommendation, ForecastFactors, ForecastResult},
    utils::moon::moon_phase,
};

use super::weather::fetch_current_weather;

/// In-memory model registry for the application (lazily initialized)
pub static MODEL_REGISTRY: std::sync::LazyLock<ModelRegistry> =
    std::sync::LazyLock::new(|| ModelRegistry::new());

/// Build a forecast using ML model and current weather.
pub async fn build_forecast_ml(
    http: &reqwest::Client,
    lat: f64,
    lon: f64,
    fish: Option<&str>,
) -> anyhow::Result<ForecastResult> {
    let weather = fetch_current_weather(http, lat, lon)
        .await
        .context("weather fetch failed")?;

    let hour = weather.time.hour() as u32;
    let day_of_year = weather.time.ordinal() + 1;
    let moon = moon_phase(weather.time);

    let features = create_features(
        weather.temperature_c,
        weather.pressure_hpa,
        weather.wind_speed_ms,
        weather.wind_direction_deg,
        weather.precipitation_mm,
        hour,
        day_of_year,
        moon,
        lat,
        None, // Cloud cover not in current weather type
        None, // Humidity not in current weather type
    );

    let prediction = MODEL_REGISTRY.predict(&features).await;

    // Convert ML prediction to forecast format
    let probability = prediction;
    let confidence = 0.6; // Default confidence

    // Generate factor scores
    let pressure_score = score_pressure(weather.pressure_hpa);
    let temperature_score = score_temperature(weather.temperature_c);
    let time_of_day_score = score_time_of_day(hour);
    let wind_score = score_wind(weather.wind_speed_ms);
    let moon_score = score_moon(moon);
    let other_score = 0.5_f64;

    let factors = ForecastFactors {
        pressure_score,
        temperature_score,
        time_of_day_score,
        wind_score,
        moon_score,
        other_score,
    };

    let recommended_baits = generate_bait_recommendations(fish, probability);

    let explanation = format!(
        "ML model predicts {:.0}% bite probability based on current conditions: {:.1}°C, {} hPa pressure, {:.1} m/s wind, {} moon phase.",
        probability * 100.0,
        weather.temperature_c,
        weather.pressure_hpa,
        weather.wind_speed_ms,
        if moon < 0.1 || moon > 0.9 { "new/full" } else { "quarter" }
    );

    Ok(ForecastResult {
        probability,
        confidence,
        factors,
        explanation,
        recommended_baits,
        best_time: get_best_time(hour),
        weather,
        moon_phase: moon,
    })
}

/// Get detailed ML prediction with factor breakdown
pub async fn get_detailed_prediction(
    http: &reqwest::Client,
    lat: f64,
    lon: f64,
) -> anyhow::Result<PredictionResult> {
    let weather = fetch_current_weather(http, lat, lon)
        .await
        .context("weather fetch failed")?;

    let hour = weather.time.hour() as u32;
    let day_of_year = weather.time.ordinal() + 1;
    let moon = moon_phase(weather.time);

    let features = create_features(
        weather.temperature_c,
        weather.pressure_hpa,
        weather.wind_speed_ms,
        weather.wind_direction_deg,
        weather.precipitation_mm,
        hour,
        day_of_year,
        moon,
        lat,
        None,
        None,
    );

    let model = MODEL_REGISTRY.get().await;
    Ok(model.predict_detailed(&features))
}

/// Initialize the ML model with default weights
pub async fn initialize_model() {
    let mut model = MODEL_REGISTRY.get_mut().await;

    // Add some default training data based on fishing heuristics
    let default_samples = generate_default_training_data();
    model.add_samples(default_samples);

    // Train the model
    model.train();

    tracing::info!("ML model initialized with {} training samples", model.get_stats().n_samples);
}

/// Generate default training data based on fishing knowledge
fn generate_default_training_data() -> Vec<TrainingSample> {
    let mut samples = Vec::new();

    // Optimal conditions
    for hour in [6, 7, 8, 18, 19, 20] {
        for temp in [15.0, 18.0, 20.0, 22.0] {
            for pressure in [1010.0, 1013.0, 1015.0, 1020.0] {
                for moon in [0.0, 0.1, 0.9, 1.0] {
                    samples.push(TrainingSample {
                        features: create_features(
                            temp, pressure, 3.0, Some(180.0), None,
                            hour, 120, moon, 52.0, Some(0.3), Some(60.0),
                        ),
                        bite_intensity: 0.8,
                        success_rate: 0.75,
                    });
                }
            }
        }
    }

    // Suboptimal conditions
    for hour in [10, 11, 12, 13, 14] {
        for temp in [5.0, 8.0, 10.0, 30.0, 32.0] {
            for pressure in [980.0, 990.0, 1030.0, 1040.0] {
                samples.push(TrainingSample {
                    features: create_features(
                        temp, pressure, 12.0, Some(270.0), Some(5.0),
                        hour, 30, 0.25, 52.0, Some(0.8), Some(85.0),
                    ),
                    bite_intensity: 0.2,
                    success_rate: 0.15,
                });
            }
        }
    }

    samples
}

/// Get feature importance from current model
pub async fn get_feature_importance() -> FeatureImportance {
    let model = MODEL_REGISTRY.get().await;
    FeatureImportance::from_model(&model)
}

/// Add user catch data for model improvement
pub async fn add_training_sample(sample: TrainingSample) {
    let mut model = MODEL_REGISTRY.get_mut().await;
    model.add_sample(sample);

    // Retrain periodically or after threshold
    if model.get_stats().n_samples % 100 == 0 {
        model.train();
    }
}

// Scoring functions from original prediction.rs
fn clamp01(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

fn score_pressure(pressure_hpa: f64) -> f64 {
    if (1000.0..=1020.0).contains(&pressure_hpa) {
        0.9
    } else if (990.0..=1030.0).contains(&pressure_hpa) {
        0.6
    } else {
        0.3
    }
}

fn score_temperature(temp_c: f64) -> f64 {
    if (15.0..=22.0).contains(&temp_c) {
        0.85
    } else if (10.0..=28.0).contains(&temp_c) {
        0.55
    } else {
        0.25
    }
}

fn score_time_of_day(hour: u32) -> f64 {
    if (5..=9).contains(&hour) || (18..=21).contains(&hour) {
        0.8
    } else if (11..=16).contains(&hour) {
        0.3
    } else {
        0.5
    }
}

fn score_wind(wind_ms: f64) -> f64 {
    if (2.0..=7.0).contains(&wind_ms) {
        0.8
    } else if wind_ms < 2.0 {
        0.4
    } else if wind_ms <= 12.0 {
        0.55
    } else {
        0.2
    }
}

fn score_moon(phase: f64) -> f64 {
    if phase <= 0.1 || phase >= 0.9 {
        0.8
    } else if (0.4..=0.6).contains(&phase) {
        0.6
    } else {
        0.5
    }
}

fn generate_bait_recommendations(fish: Option<&str>, probability: f64) -> Vec<BaitRecommendation> {
    match fish {
        Some("trout") | Some("rainbow") => vec![
            BaitRecommendation { name: "spinner".to_string(), score: probability * 0.9 },
            BaitRecommendation { name: "fly".to_string(), score: probability * 0.85 },
            BaitRecommendation { name: "worm".to_string(), score: probability * 0.7 },
        ],
        Some("pike") | Some("zander") => vec![
            BaitRecommendation { name: "dead bait".to_string(), score: probability * 0.9 },
            BaitRecommendation { name: "lure".to_string(), score: probability * 0.8 },
            BaitRecommendation { name: "fish".to_string(), score: probability * 0.75 },
        ],
        Some("carp") | Some("grass carp") => vec![
            BaitRecommendation { name: "boilie".to_string(), score: probability * 0.9 },
            BaitRecommendation { name: "corn".to_string(), score: probability * 0.8 },
            BaitRecommendation { name: "particles".to_string(), score: probability * 0.7 },
        ],
        Some("perch") | Some("roach") => vec![
            BaitRecommendation { name: "maggot".to_string(), score: probability * 0.9 },
            BaitRecommendation { name: "worm".to_string(), score: probability * 0.8 },
            BaitRecommendation { name: "corn".to_string(), score: probability * 0.6 },
        ],
        _ => vec![
            BaitRecommendation { name: "worm".to_string(), score: probability * 0.7 },
            BaitRecommendation { name: "spinner".to_string(), score: probability * 0.6 },
            BaitRecommendation { name: "corn".to_string(), score: probability * 0.5 },
        ],
    }
}

fn get_best_time(hour: u32) -> String {
    if (5..=9).contains(&hour) {
        "Now - Excellent morning bite!".to_string()
    } else if (10..=15).contains(&hour) {
        "Best time: Early morning or evening".to_string()
    } else if (16..=21).contains(&hour) {
        "Now - Good evening bite expected".to_string()
    } else {
        "Best time: Dawn or dusk (5-9 or 18-21)".to_string()
    }
}
