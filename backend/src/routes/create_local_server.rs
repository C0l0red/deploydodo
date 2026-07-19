use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dependencies::Dependencies;
use crate::error::{AppError, AppResult};
use crate::extractors::{Auth, RequestJson};
use crate::services::types::{self, VariableKey};
use crate::validation::NonEmptyString;

#[derive(Deserialize, ToSchema)]
pub struct CreateLocalServerRequest {
    pub name: NonEmptyString,
}

#[derive(Serialize, ToSchema)]
pub struct CreateLocalServerResponse {
    pub id: i64,
    pub name: String,
    #[serde(rename = "serverType")]
    pub server_type: types::ServerType,
    pub hostname: String,
    pub port: u16,
}

// ── Handler ───────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/setup/server/local",
    request_body = CreateLocalServerRequest,
    params(
        ("Authorization" = String, Header, description = "authorization token")
    ),
    responses(
        (status = 201, description = "Local server created", body = CreateLocalServerResponse),
    ),
    tag = "setup"
)]
pub async fn create_local_server(
    _: Auth,
    State(deps): State<Dependencies>,
    RequestJson(request): RequestJson<CreateLocalServerRequest>,
) -> AppResult<(StatusCode, Json<CreateLocalServerResponse>)> {
    let count = deps.server_service.count_local_servers().await?;
    if count > 0 {
        return Err(AppError::LocalServerAlreadyExists);
    }

    let server = deps
        .server_service
        .create_local_server(&request.name)
        .await?;

    tracing::info!(id = %server.id(), "local server created");

    deps.variables_service
        .set_value(VariableKey::IsLocalServerSetup, true)
        .await?;

    deps.variables_service
        .set_value(VariableKey::IsServerSetup, true)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateLocalServerResponse {
            id: *server.id(),
            name: server.name().to_owned(),
            server_type: server.server_type(),
            hostname: server.hostname().to_owned(),
            port: server.ssh_port().to_owned(),
        }),
    ))
}
