use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{error::AppResult, extractors::MaybeAuth};

#[derive(Serialize, ToSchema)]
pub struct ValidateSessionResponse {
    pub valid: bool,
}

#[utoipa::path(
    get,
    path = "/api/auth/validate",
    responses(
        (status = 200, description = "Session is valid", body = ValidateSessionResponse),
    ),
    tag = "auth"
)]
pub async fn validate_session(
    MaybeAuth(user): MaybeAuth,
) -> AppResult<Json<ValidateSessionResponse>> {
    Ok(Json(ValidateSessionResponse {
        valid: user.is_some(),
    }))
}
