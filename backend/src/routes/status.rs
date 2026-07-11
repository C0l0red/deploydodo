use axum::{extract::State, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{dependencies::Dependencies, error::AppResult};

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
}

#[utoipa::path(
    get,
    path = "/api/status",
    responses(
        (status = 200, description = "Config status", body = StatusResponse),
    ),
    tag = "status"
)]
pub async fn status(State(deps): State<Dependencies>) -> AppResult<Json<StatusResponse>> {
    let is_admin_onboarded = deps
        .variables_service
        .get("is_admin_onboarded")
        .await?
        .is_some_and(|v| v == "true");

    let is_server_setup = deps
        .variables_service
        .get("is_server_setup")
        .await?
        .is_some_and(|v| v == "true");

    let is_project_setup = deps
        .variables_service
        .get("is_project_setup")
        .await?
        .is_some_and(|v| v == "true");

    Ok(Json(StatusResponse {
        is_admin_onboarded,
        is_server_setup,
        is_project_setup,
        is_onboarding_complete: is_admin_onboarded && is_server_setup && is_project_setup,
    }))
}
