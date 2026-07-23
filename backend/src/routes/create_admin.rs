use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::{AppError, AppResult};
use crate::extractors::RequestJson;
use crate::new_types::{HashedPassword, NonEmptyString, PlainPassword};
use crate::services::types::VariableKey;
use crate::services::user_service::{UserId, UserInput};
use crate::{dependencies::Dependencies, services::types::AccountType};

#[derive(Deserialize, ToSchema)]
pub struct CreateAdminRequest {
    pub name: NonEmptyString,
    pub email: NonEmptyString,
    pub password: PlainPassword,
}

#[derive(Serialize, ToSchema)]
pub struct AdminResponse {
    pub id: UserId,
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
    RequestJson(request): RequestJson<CreateAdminRequest>,
) -> AppResult<(StatusCode, Json<AdminResponse>)> {
    let count = deps.user_service.count_users().await?;
    if count > 0 {
        return Err(AppError::AdminAlreadyConfigured);
    }

    let hashed_password = HashedPassword::hash(&request.password)?;
    let user_input = UserInput::admin(request, hashed_password);
    let user = deps.user_service.create_user(user_input).await?;

    let session_token = deps.session_service.create_session(user.id).await?;
    deps.variables_service
        .set_value(VariableKey::IsAdminOnboarded, true)
        .await?;

    tracing::info!(email = %user.email, id = %user.id, "admin user created");

    Ok((
        StatusCode::CREATED,
        Json(AdminResponse {
            id: user.id,
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

    // FIXME: Now that we have unit tests that ensure the types validate properly, do we still need tests like this?
    #[sqlx::test]
    async fn create_admin_fails_if_name_is_missing(db: Pool<Postgres>) {
        let app = App::register_route(db, post(create_admin)).await;

        app.post()
            .json(&json!({}))
            .await
            .assert_status_unprocessable_entity()
            .assert_text_contains("missing field `name`");
    }

    #[sqlx::test]
    async fn create_admin_fails_if_email_is_missing(db: Pool<Postgres>) {
        let app = App::register_route(db, post(create_admin)).await;

        app.post()
            .json(&json!({"name": "Test user"}))
            .await
            .assert_status_unprocessable_entity()
            .assert_text_contains("missing field `email`");
    }

    #[sqlx::test]
    async fn create_admin_fails_if_password_is_missing(db: Pool<Postgres>) {
        let app = App::register_route(db, post(create_admin)).await;

        app.post()
            .json(&json!({"name": "Test user", "email": "test@user.com"}))
            .await
            .assert_status_unprocessable_entity()
            .assert_text_contains("missing field `password`");
    }

    #[sqlx::test]
    async fn create_admin_fails_if_name_is_blank(db: Pool<Postgres>) {
        let app = App::register_route(db, post(create_admin)).await;

        app.post()
            .json(&json!({"name": "", "email": "test@user.com", "password": ""}))
            .await
            .assert_status_unprocessable_entity()
            .assert_json_contains(&json!({
                "message": "name: must not be empty"
            }));
    }

    #[sqlx::test]
    async fn create_admin_fails_if_email_is_blank(db: Pool<Postgres>) {
        let app = App::register_route(db, post(create_admin)).await;

        app.post()
            .json(&json!({"name": "Test user", "email": "", "password": ""}))
            .await
            .assert_status_unprocessable_entity()
            .assert_json_contains(&json!({
                "message": "email: must not be empty"
            }));
    }

    #[sqlx::test]
    async fn create_admin_fails_if_password_is_blank(db: Pool<Postgres>) {
        let app = App::register_route(db, post(create_admin)).await;

        app.post()
            .json(&json!({"name": "Test user", "email": "test@user.com", "password": ""}))
            .await
            .assert_status_unprocessable_entity()
            .assert_json_contains(&json!({
                "message": "password: must be at least 8 characters"
            }));
    }
}
