use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::dependencies::Dependencies;
use crate::error::AppError;
use crate::services::terminal_service::TerminalService;
use crate::services::types::{ContainerInfo, ServerType};

#[derive(Serialize, ToSchema)]
pub struct ContainerListResponse {
    pub containers: Vec<ContainerInfo>,
}

#[utoipa::path(
    get,
    path = "/api/servers/{server_id}/containers",
    params(
        ("server_id" = i64, Path, description = "Server ID"),
    ),
    responses(
        (status = 200, description = "List of running containers", body = ContainerListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Server not found"),
    ),
    tag = "servers"
)]
pub async fn list_containers(
    State(deps): State<Dependencies>,
    Path(server_id): Path<i64>,
) -> Result<(StatusCode, Json<ContainerListResponse>), AppError> {
    let servers = deps.server_service.list_servers().await?;
    let server = servers
        .into_iter()
        .find(|s| s.id == server_id)
        .ok_or(AppError::Validation("Server not found".into()))?;

    let ssh_key = if server.server_type == ServerType::Remote {
        let key_id = server.ssh_key_id.ok_or(AppError::Validation(
            "No SSH key for remote server".into(),
        ))?;
        Some(deps.ssh_service.get_key_by_id(key_id).await?)
    } else {
        None
    };

    let session = TerminalService::connect(&server, ssh_key.as_ref()).await?;
    let containers = TerminalService::list_containers(&session.docker).await?;

    Ok((
        StatusCode::OK,
        Json(ContainerListResponse { containers }),
    ))
}
