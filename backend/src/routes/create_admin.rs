use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::{AppError, AppResult};
use crate::{
    dependencies::Dependencies,
    services::types::{AccountType, User},
};

#[derive(Deserialize, ToSchema)]
pub struct CreateAdminRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl CreateAdminRequest {
    pub fn validate(&self) -> AppResult<()> {
        if self.name.trim().is_empty() {
            return Err(AppError::Validation("Name is required".into()));
        }
        if self.email.trim().is_empty() {
            return Err(AppError::Validation("Email is required".into()));
        }
        if self.password.len() < 8 {
            return Err(AppError::Validation(
                "Password must be at least 8 characters".into(),
            ));
        }
        Ok(())
    }

    pub fn to_admin_user(self) -> User {
        User {
            id: None,
            name: self.name,
            email: self.email,
            password_hash: self.password,
            account_type: AccountType::Admin,
            created_at: Utc::now(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct AdminResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "sessionToken")]
    pub session_token: String,
}

#[utoipa::path(
    post,
    path = "/api/setup/admin",
    request_body = CreateAdminRequest,
    responses(
        (status = 201, description = "Admin user created", body = AdminResponse),
        (status = 409, description = "Admin already configured"),
        (status = 422, description = "Validation error"),
    ),
    tag = "setup"
)]
pub async fn create_admin(
    State(deps): State<Dependencies>,
    Json(request): Json<CreateAdminRequest>,
) -> AppResult<(StatusCode, Json<AdminResponse>)> {
    request.validate()?;

    let count = deps.user_service.count_users().await?;
    if count > 0 {
        return Err(AppError::AdminAlreadyConfigured);
    }

    let user = deps
        .user_service
        .create_user(request.to_admin_user())
        .await?;

    let user_id = user.id.unwrap();
    let session_token = deps.session_service.create_session(user_id).await?;
    deps.variables_service
        .set("is_admin_onboarded", "true")
        .await?;

    tracing::info!(email = %user.email, id = %user_id, "admin user created");

    Ok((
        StatusCode::CREATED,
        Json(AdminResponse {
            id: user_id,
            name: user.name,
            email: user.email,
            account_type: user.account_type,
            created_at: user.created_at.to_rfc3339(),
            session_token,
        }),
    ))
}
