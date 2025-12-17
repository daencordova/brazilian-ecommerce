use crate::error::AppError;
use std::env;
use std::time::Duration;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub cors: CorsConfig,
}

#[derive(Clone)]
pub struct CorsConfig {
    pub allowed_origins: AllowOrigin,
    pub allow_credentials: bool,
    pub max_age_seconds: u64,
}

pub fn load_config() -> Result<AppConfig, AppError> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| AppError::ConfigError("DATABASE_URL must be set".to_string()))?;

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .map_err(|e| AppError::ConfigError(format!("Invalid PORT: {}", e)))?;

    Ok(AppConfig {
        database_url,
        port,
        cors: load_cors_config()?,
    })
}

pub fn load_cors_config() -> Result<CorsConfig, AppError> {
    let allowed_origins_str = env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string());

    let allowed_origins = if allowed_origins_str == "*" {
        Any.into()
    } else {
        let origins: Vec<_> = allowed_origins_str
            .split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<_, _>>()
            .map_err(|e| AppError::ConfigError(format!("Invalid CORS origin: {}", e)))?;
        AllowOrigin::list(origins)
    };

    Ok(CorsConfig {
        allowed_origins,
        allow_credentials: env::var("CORS_ALLOW_CREDENTIALS")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true),
        max_age_seconds: env::var("CORS_MAX_AGE")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .unwrap_or(3600),
    })
}

pub fn create_cors_layer(config: CorsConfig) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(config.allowed_origins)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(config.allow_credentials)
        .max_age(Duration::from_secs(config.max_age_seconds))
}
