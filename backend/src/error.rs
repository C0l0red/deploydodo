use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("admin user already configured")]
    AdminAlreadyConfigured,

    #[error("password hashing failed")]
    PasswordHash,

    #[error("internal error occurred: {0}")]
    InternalServerError(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("not found error: {0}")]
    NotFound(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("a local server already exists")]
    LocalServerAlreadyExists,

    #[error("missing key secret")]
    MissingKeySecret,

    #[error("Connection error: {0}")]
    Ssh(#[from] dodosh::ShellError),

    #[error("job not found")]
    JobNotFound,

    #[error("local docker error: {0}")]
    LocalDockerConnect(String),

    #[error("remote docker via SSH error: {0}")]
    RemoteDockerConnect(String),

    #[error("docker operation error: {0}")]
    DockerOperation(String),
}

impl AppError {
    pub fn validation(message: &str) -> Self {
        Self::Validation(message.to_owned())
    }

    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest(message.to_owned())
    }

    pub fn not_found(message: &str) -> Self {
        Self::NotFound(message.to_owned())
    }

    pub fn internal_server_error(message: &str) -> Self {
        Self::InternalServerError(message.to_owned())
    }
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
            AppError::InternalServerError(_) => {
                tracing::error!("internal error: {}", self.to_string());
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
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
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
            AppError::LocalDockerConnect(e) => {
                tracing::error!("local docker connect error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::RemoteDockerConnect(e) => {
                tracing::error!("remote docker connect error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::DockerOperation(e) => {
                tracing::error!("docker operation error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        (status, Json(json!({ "message": message }))).into_response()
    }
}
