use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::extractors::RequestJson;
use crate::new_types::{NonEmptyString, PlainPassword};
use crate::{
    dependencies::Dependencies,
    error::{AppError, AppResult},
};

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: NonEmptyString,
    pub password: PlainPassword,
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
    ),
    tag = "auth"
)]
pub async fn login(
    State(deps): State<Dependencies>,
    RequestJson(request): RequestJson<LoginRequest>,
) -> AppResult<(StatusCode, Json<LoginResponse>)> {
    let user = deps
        .user_service
        .get_by_email(&request.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    user.password_hash.verify(&request.password)?;

    let session_token = deps.session_service.create_session(user.id).await?;

    Ok((StatusCode::OK, Json(LoginResponse { session_token })))
}
