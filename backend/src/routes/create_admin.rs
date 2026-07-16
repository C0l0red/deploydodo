use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::{AppError, AppResult};
use crate::services::types::VariableKey;
use crate::{dependencies::Dependencies, services::types::AccountType};

#[derive(Deserialize, ToSchema)]
pub struct CreateAdminRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl CreateAdminRequest {
    pub fn validate(&self) -> AppResult<()> {
        if self.name.trim().is_empty() {
            return Err(AppError::bad_request("Name is required"));
        }
        if self.email.trim().is_empty() {
            return Err(AppError::bad_request("Email is required"));
        }
        if self.password.len() < 8 {
            return Err(AppError::bad_request(
                "Password must be at least 8 characters",
            ));
        }
        Ok(())
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

    let user = deps.user_service.create_user(request.into()).await?;

    let user_id = user.id.unwrap();
    let session_token = deps.session_service.create_session(user_id).await?;
    deps.variables_service
        .set_value(VariableKey::IsAdminOnboarded, true)
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

#[cfg(test)]
mod tests {
    use axum::routing::post;
    use serde_json::json;
    use sqlx::{Pool, Postgres};

    use crate::test::App;

    use super::create_admin;

    #[sqlx::test]
    fn create_admin_fails_if_name_is_missing(db: Pool<Postgres>) {
        let app = App::register_create_admin(db).await;

        app.server
            .post("/api/setup/admin")
            .json(&json!({}))
            .await
            .assert_status_unprocessable_entity()
            .assert_text_contains("missing field `name`");
    }

    #[sqlx::test]
    fn create_admin_fails_if_email_is_missing(db: Pool<Postgres>) {
        let app = App::register_create_admin(db).await;

        app.server
            .post("/api/setup/admin")
            .json(&json!({"name": "Test user"}))
            .await
            .assert_status_unprocessable_entity()
            .assert_text_contains("missing field `email`");
    }

    #[sqlx::test]
    fn create_admin_fails_if_password_is_missing(db: Pool<Postgres>) {
        let app = App::register_create_admin(db).await;

        app.server
            .post("/api/setup/admin")
            .json(&json!({"name": "Test user", "email": "test@user.com"}))
            .await
            .assert_status_unprocessable_entity()
            .assert_text_contains("missing field `password`");
    }

    #[sqlx::test]
    fn create_admin_fails_if_name_is_blank(db: Pool<Postgres>) {
        let app = App::register_create_admin(db).await;

        app.server
            .post("/api/setup/admin")
            .json(&json!({"name": "", "email": "", "password": ""}))
            .await
            .assert_status_bad_request()
            .assert_json_contains(&json!({
                "message": "Name is required"
            }));
    }

    #[sqlx::test]
    fn create_admin_fails_if_email_is_blank(db: Pool<Postgres>) {
        let app = App::register_create_admin(db).await;

        app.server
            .post("/api/setup/admin")
            .json(&json!({"name": "Test user", "email": "", "password": ""}))
            .await
            .assert_status_bad_request()
            .assert_json_contains(&json!({
                "message": "Email is required"
            }));
    }

    #[sqlx::test]
    fn create_admin_fails_if_password_is_blank(db: Pool<Postgres>) {
        let app = App::register_create_admin(db).await;

        app.server
            .post("/api/setup/admin")
            .json(&json!({"name": "Test user", "email": "test@user.com", "password": ""}))
            .await
            .assert_status_bad_request()
            .assert_json_contains(&json!({
                "message": "Password must be at least 8 characters"
            }));
    }

    impl App {
        async fn register_create_admin(db: Pool<Postgres>) -> Self {
            App::register_route(db, "/api/setup/admin", post(create_admin)).await
        }
    }
}
