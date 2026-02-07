mod db;
mod models;
mod routes;
mod services;

use axum::{routing::get, Router};
#[cfg(not(feature = "shuttle"))]
use sqlx::SqlitePool;
#[cfg(feature = "shuttle")]
use sqlx::PgPool;
use routes::{
    auth::{login_handler, me_handler, register_handler},
    catches::{get_catches_handler, get_nearby_catches_handler, save_catch_handler},
    fish::fish_handler,
    forecast::{detailed_forecast_handler, feature_importance_handler, forecast_handler},
    health::health_handler,
    region::region_detect_handler,
    regulations::{regulations_handler, regulations_validate_handler},
    water_bodies::water_bodies_handler,
};
use services::{http_client::build_http_client, prediction::initialize_model};
#[cfg(not(feature = "shuttle"))]
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
#[cfg(not(feature = "shuttle"))]
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Shared application state for handlers.
#[derive(Clone)]
pub struct AppState {
    pub http: reqwest::Client,
    #[cfg(not(feature = "shuttle"))]
    pub db: SqlitePool,
    #[cfg(feature = "shuttle")]
    pub db: PgPool,
}

#[cfg(not(feature = "shuttle"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL is not set"))?;

    let db = db::create_pool(&database_url).await?;
    
    // Initialize ML model
    tracing::info!("Initializing ML Engine...");
    initialize_model().await;
    tracing::info!("ML Engine initialized successfully");

    let state = AppState {
        http: build_http_client()?,
        db,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Auth routes (public)
        .route("/api/v1/auth/register", axum::routing::post(register_handler))
        .route("/api/v1/auth/login", axum::routing::post(login_handler))
        .route("/api/v1/auth/me", get(me_handler))
        // Health & Forecast (public)
        .route("/api/v1/health", get(health_handler))
        .route("/api/v1/forecast", get(forecast_handler))
        .route("/api/v1/forecast/detailed", get(detailed_forecast_handler))
        .route("/api/v1/forecast/importance", get(feature_importance_handler))
        // Region & Fish (public)
        .route("/api/v1/region/detect", get(region_detect_handler))
        .route("/api/v1/fish", get(fish_handler))
        // Regulations (public)
        .route("/api/v1/regulations", get(regulations_handler))
        .route("/api/v1/regulations/validate", axum::routing::post(regulations_validate_handler))
        // Catches (public for MVP, should be protected)
        .route("/api/v1/catches", axum::routing::post(save_catch_handler))
        .route("/api/v1/catches", get(get_catches_handler))
        .route("/api/v1/catches/nearby", get(get_nearby_catches_handler))
        // Water bodies (public)
        .route("/api/v1/water-bodies", get(water_bodies_handler))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::Secret,
) -> Result<AppState, shuttle_runtime::Error> {
    use shuttle_runtime::tracing;
    
    // Initialize ML model
    tracing::info!("Initializing ML Engine...");
    initialize_model().await;
    tracing::info!("ML Engine initialized successfully");

    let database_url = secrets
        .get("DATABASE_URL")
        .expect("DATABASE_URL not set");
    
    let jwt_secret = secrets
        .get("JWT_SECRET")
        .expect("JWT_SECRET not set");
    
    // Store JWT_SECRET in env for auth module
    std::env::set_var("JWT_SECRET", jwt_secret);

    let db = db::create_pg_pool(&database_url).await;
    
    let state = AppState {
        http: build_http_client().expect("Failed to build HTTP client"),
        db,
    };

    Ok(state)
}

#[cfg(feature = "shuttle")]
use routes::Router;

#[cfg(feature = "shuttle")]
#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for AppState {
    type StartupError = anyhow::Error;
    
    async fn start(&self, _: &shuttle_runtime::ProjectName) -> Result<Router, Self::StartupError> {
        let cors = tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any);

        let app = Router::new()
            // Auth routes (public)
            .route("/api/v1/auth/register", axum::routing::post(register_handler))
            .route("/api/v1/auth/login", axum::routing::post(login_handler))
            .route("/api/v1/auth/me", get(me_handler))
            // Health & Forecast (public)
            .route("/api/v1/health", get(health_handler))
            .route("/api/v1/forecast", get(forecast_handler))
            .route("/api/v1/forecast/detailed", get(detailed_forecast_handler))
            .route("/api/v1/forecast/importance", get(feature_importance_handler))
            // Region & Fish (public)
            .route("/api/v1/region/detect", get(region_detect_handler))
            .route("/api/v1/fish", get(fish_handler))
            // Regulations (public)
            .route("/api/v1/regulations", get(regulations_handler))
            .route("/api/v1/regulations/validate", axum::routing::post(regulations_validate_handler))
            // Catches (public for MVP, should be protected)
            .route("/api/v1/catches", axum::routing::post(save_catch_handler))
            .route("/api/v1/catches", get(get_catches_handler))
            .route("/api/v1/catches/nearby", get(get_nearby_catches_handler))
            // Water bodies (public)
            .route("/api/v1/water-bodies", get(water_bodies_handler))
            .with_state(self.clone())
            .layer(cors);

        Ok(app)
    }
}
