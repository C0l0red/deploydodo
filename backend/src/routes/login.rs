use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

    user.verify_password(&request.password)?;

    if let Some(user_id) = user.id {
        let session_token = deps.session_service.create_session(user_id).await?;

        Ok((StatusCode::OK, Json(LoginResponse { session_token })))
    } else {
        Err(AppError::InternalServerError(
            "id is None in User type".to_string(),
        ))
    }
}
