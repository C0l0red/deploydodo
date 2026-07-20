use axum::{extract::State, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    dependencies::Dependencies,
    error::AppResult,
    services::types::{VariableKey, VariableValueByKey},
};

#[derive(Serialize, ToSchema)]
pub struct StatusResponse {
    #[serde(rename = "isAdminOnboarded")]
    pub is_admin_onboarded: bool,
    #[serde(rename = "isServerSetup")]
    pub is_server_setup: bool,
    #[serde(rename = "isProjectSetup")]
    pub is_project_setup: bool,
    #[serde(rename = "isOnboardingComplete")]
    pub is_onboarding_complete: bool,
    #[serde(rename = "isLocalServerSetup")]
    pub is_local_server_setup: bool,
}

// FIXME: This seems quite roundabout and can be simplified with a struct returned
#[utoipa::path(
    get,
    path = "/api/status",
    params(
        ("Authorization" = String, Header, description = "authorization token")
    ),
    responses(
        (status = 200, description = "Config status", body = StatusResponse),
    ),
    tag = "status"
)]
pub async fn status(State(deps): State<Dependencies>) -> AppResult<Json<StatusResponse>> {
    let variable_keys = vec![
        VariableKey::IsAdminOnboarded,
        VariableKey::IsServerSetup,
        VariableKey::IsLocalServerSetup,
        VariableKey::IsProjectSetup,
    ];

    let variables = deps.variables_service.get_all(variable_keys).await?;

    // FIXME: Are these variables truly supposed to be optional?
    let is_admin_onboarded = variables
        .get_boolean(VariableKey::IsAdminOnboarded)
        .unwrap_or(false);
    let is_server_setup = variables
        .get_boolean(VariableKey::IsServerSetup)
        .unwrap_or(false);
    let is_project_setup = variables
        .get_boolean(VariableKey::IsProjectSetup)
        .unwrap_or(false);
    let is_local_server_setup = variables
        .get_boolean(VariableKey::IsLocalServerSetup)
        .unwrap_or(false);

    Ok(Json(StatusResponse {
        is_admin_onboarded,
        is_server_setup,
        is_project_setup,
        is_local_server_setup,
        is_onboarding_complete: is_admin_onboarded && is_server_setup && is_project_setup,
    }))
}

#[cfg(test)]
mod tests {
    use axum::routing::get;
    use serde_json::json;
    use sqlx::{Pool, Postgres};

    use super::status;
    use crate::{services::types::VariableKey, test::App};

    #[sqlx::test]
    fn status_returns_false_for_all_variables(db: Pool<Postgres>) {
        let app = App::register_route(db, get(status)).await;

        app.get().await.assert_status_ok().assert_json(&json!({
            "isAdminOnboarded": false,
            "isServerSetup": false,
            "isProjectSetup": false,
            "isOnboardingComplete": false,
            "isLocalServerSetup": false,
        }));
    }

    #[sqlx::test]
    fn status_returns_true_for_set_var(db: Pool<Postgres>) {
        let app = App::register_route(db, get(status)).await;

        let _ = app
            .deps
            .variables_service
            .set_value(VariableKey::IsAdminOnboarded, true)
            .await;

        app.get()
            .await
            .assert_status_ok()
            .assert_json_contains(&json!({
                "isAdminOnboarded": true,
            }));
    }
}
