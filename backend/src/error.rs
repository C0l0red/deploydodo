use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("admin user already configured")]
    AdminAlreadyConfigured,

    #[error("password hashing failed")]
    PasswordHash,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("a local server already exists")]
    LocalServerAlreadyExists,

    #[error("missing key secret")]
    MissingKeySecret,

    #[error("Connection error: {0}")]
    Ssh(#[from] dodosh::SshError),

    #[error("job not found")]
    JobNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("database error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::AdminAlreadyConfigured => (StatusCode::CONFLICT, self.to_string()),
            AppError::PasswordHash => {
                tracing::error!("password hashing failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::LocalServerAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            AppError::MissingKeySecret => {
                tracing::error!("missing key secret");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::Ssh(e) => {
                tracing::error!("dodosh error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::JobNotFound => (StatusCode::NOT_FOUND, self.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
