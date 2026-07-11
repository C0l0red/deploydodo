use axum::{extract::State, http::HeaderMap, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    dependencies::Dependencies,
    error::{AppError, AppResult},
};

#[derive(Serialize, ToSchema)]
pub struct ValidateSessionResponse {
    pub valid: bool,
}

#[utoipa::path(
    get,
    path = "/api/auth/validate",
    responses(
        (status = 200, description = "Session is valid", body = ValidateSessionResponse),
        (status = 401, description = "Invalid or missing session token"),
    ),
    tag = "auth"
)]
pub async fn validate_session(
    State(deps): State<Dependencies>,
    headers: HeaderMap,
) -> AppResult<Json<ValidateSessionResponse>> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    if !deps.session_service.validate_session(token).await? {
        return Err(AppError::Unauthorized);
    }

    Ok(Json(ValidateSessionResponse { valid: true }))
}
