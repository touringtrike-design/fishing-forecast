mod db;
mod models;
mod routes;
mod services;

use axum::{routing::get, Router};
use routes::{
    catches::{get_catches_handler, get_nearby_catches_handler, save_catch_handler},
    fish::fish_handler,
    forecast::forecast_handler,
    health::health_handler,
    region::region_detect_handler,
    regulations::{regulations_handler, regulations_validate_handler},
    water_bodies::water_bodies_handler,
};
use services::http_client::build_http_client;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::SqlitePool;

/// Shared application state for handlers.
#[derive(Clone)]
pub struct AppState {
    pub http: reqwest::Client,
    pub db: SqlitePool,
}

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
    // Migration support will be added later
    // For now, we use a simple in-memory database for development

    let state = AppState {
        http: build_http_client()?,
        db,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/v1/health", get(health_handler))
        .route("/api/v1/forecast", get(forecast_handler))
        .route("/api/v1/region/detect", get(region_detect_handler))
        .route("/api/v1/fish", get(fish_handler))
        .route("/api/v1/regulations", get(regulations_handler))
        .route("/api/v1/regulations/validate", axum::routing::post(regulations_validate_handler))
        .route("/api/v1/catches", axum::routing::post(save_catch_handler))
        .route("/api/v1/catches", get(get_catches_handler))
        .route("/api/v1/catches/nearby", get(get_nearby_catches_handler))
        .route("/api/v1/water-bodies", get(water_bodies_handler))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
