use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::services::user_service::UserService;
use crate::{dependencies::Dependencies, error::AppError};

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    #[serde(rename = "sessionToken")]
    pub session_token: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
    ),
    tag = "auth"
)]
pub async fn login(
    State(deps): State<Dependencies>,
    Json(request): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    let user = deps
        .user_service
        .get_by_email(&request.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let valid =
        UserService::verify_password(&request.password, &user.password_hash).map_err(|_| AppError::InvalidCredentials)?;

    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let session_token = deps
        .session_service
        .create_session(user.id.ok_or(AppError::InvalidCredentials)?)
        .await?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse { session_token }),
    ))
}
