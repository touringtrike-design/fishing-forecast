use anyhow::{Context, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::AppState;

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub email: String,
    pub exp: usize,         // Expiration timestamp
    pub iat: usize,         // Issued at timestamp
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Register request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub country_code: Option<String>,
    pub language: Option<String>,
}

/// Auth response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

/// User response (without sensitive data)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub country_code: String,
    pub language: String,
    pub created_at: String,
}

/// User in database
#[derive(Debug, FromRow)]
struct DbUser {
    id: String,
    email: String,
    password_hash: String,
    country_code: Option<String>,
    language: Option<String>,
    created_at: chrono::DateTime<Utc>,
}

/// Get JWT secret from environment
fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set, using default dev secret");
        "development-secret-key-change-in-production".to_string()
    })
}

/// Hash password using bcrypt
pub async fn hash_password(password: &str) -> Result<String> {
    let password = password.to_string();
    let result = tokio::task::spawn_blocking(move || {
        hash(&password, DEFAULT_COST)
    })
    .await
    .map_err(|e| anyhow::anyhow!("Thread panic: {}", e))?;
    
    result
        .map_err(anyhow::Error::msg)
}

/// Verify password
pub async fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let password = password.to_string();
    let hash = hash.to_string();
    let result = tokio::task::spawn_blocking(move || {
        verify(&password, &hash)
    })
    .await
    .map_err(|e| anyhow::anyhow!("Thread panic: {}", e))?;
    
    result
        .map_err(anyhow::Error::msg)
}

/// Generate JWT token
pub fn generate_token(user_id: &str, email: &str) -> Result<String> {
    let secret = get_jwt_secret();
    let now = Utc::now();
    let duration = Duration::hours(24 * 7); // 7 days expiry
    
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: (now + duration).timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .context("Failed to encode JWT token")
}

/// Validate JWT token
pub fn validate_token(token: &str) -> Result<Claims> {
    let secret = get_jwt_secret();
    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .context("Invalid JWT token")
    .map(|data| data.claims)
}

/// Register new user
pub async fn register(
    state: &AppState,
    request: RegisterRequest,
) -> Result<AuthResponse> {
    // Check if user already exists
    let existing: Option<DbUser> = sqlx::query_as(
        "SELECT id, email, password_hash, country_code, language, created_at FROM users WHERE email = ?1"
    )
    .bind(&request.email)
    .fetch_optional(&state.db)
    .await?;
    
    if existing.is_some() {
        return Err(anyhow::anyhow!("User with this email already exists"));
    }
    
    // Hash password
    let password_hash = hash_password(&request.password).await?;
    
    // Create user
    let user_id = Uuid::new_v4().to_string();
    let country_code = request.country_code.unwrap_or_else(|| "UA".to_string());
    let language = request.language.unwrap_or_else(|| "uk".to_string());
    
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, country_code, language) VALUES (?1, ?2, ?3, ?4, ?5)"
    )
    .bind(&user_id)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(&country_code)
    .bind(&language)
    .execute(&state.db)
    .await
    .context("Failed to create user")?;
    
    // Generate token
    let token = generate_token(&user_id, &request.email)?;
    
    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            email: request.email,
            country_code,
            language,
            created_at: Utc::now().to_rfc3339(),
        },
    })
}

/// Login user
pub async fn login(
    state: &AppState,
    request: LoginRequest,
) -> Result<AuthResponse> {
    // Find user
    let user: DbUser = sqlx::query_as(
        "SELECT id, email, password_hash, country_code, language, created_at FROM users WHERE email = ?1"
    )
    .bind(&request.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;
    
    // Verify password
    let valid = verify_password(&request.password, &user.password_hash).await?;
    if !valid {
        return Err(anyhow::anyhow!("Invalid email or password"));
    }
    
    // Generate token
    let token = generate_token(&user.id, &user.email)?;
    
    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            country_code: user.country_code.unwrap_or_default(),
            language: user.language.unwrap_or_default(),
            created_at: user.created_at.to_rfc3339(),
        },
    })
}

/// Get user by ID
pub async fn get_user_by_id(
    state: &AppState,
    user_id: &str,
) -> Result<Option<UserResponse>> {
    let user: Option<DbUser> = sqlx::query_as(
        "SELECT id, email, password_hash, country_code, language, created_at FROM users WHERE id = ?1"
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;
    
    Ok(user.map(|u| UserResponse {
        id: u.id,
        email: u.email,
        country_code: u.country_code.unwrap_or_default(),
        language: u.language.unwrap_or_default(),
        created_at: u.created_at.to_rfc3339(),
    }))
}
