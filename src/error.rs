use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use sqlx::migrate::MigrateError;
use tracing::error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    MigrationError(MigrateError),
    NotFound,
    ConfigError(String),
    ValidationError(validator::ValidationErrors),
    NoChangesToUpdate,
    AlreadyExists(String),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::DatabaseError(error)
    }
}

impl From<MigrateError> for AppError {
    fn from(error: MigrateError) -> Self {
        AppError::MigrationError(error)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(error: validator::ValidationErrors) -> Self {
        AppError::ValidationError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource Not Found".to_string()),
            AppError::ValidationError(e) => {
                (StatusCode::BAD_REQUEST, format!("Validation error: {}", e))
            }
            AppError::NoChangesToUpdate => (
                StatusCode::BAD_REQUEST,
                "No valid fields provided for update.".to_string(),
            ),
            AppError::AlreadyExists(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::DatabaseError(e) => {
                error!("Database Error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database Operation Failed".to_string(),
                )
            }
            AppError::MigrationError(e) => {
                error!("Migration Error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database Migration Failed".to_string(),
                )
            }
            AppError::ConfigError(e) => {
                error!("Configuration Error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Configuration Error: {}", e),
                )
            }
        };

        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}

pub fn map_db_error(e: sqlx::Error, resource_name: &str) -> AppError {
    if let sqlx::Error::Database(db_err) = &e {
        if db_err.code().as_deref() == Some("23505") {
            return AppError::AlreadyExists(format!("{} already exists", resource_name));
        }
    }
    AppError::DatabaseError(e)
}
