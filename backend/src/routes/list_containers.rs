use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::dependencies::Dependencies;
use crate::error::AppError;
use crate::services::terminal_service::connect;
use crate::services::terminal_service::resolve_ssh_key;
use crate::services::types::ContainerInfo;

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
    let server = deps.server_service.get_server_by_id(server_id).await?;
    let ssh_key = resolve_ssh_key(&server, &deps.ssh_service).await?;
    let session = connect(&server, ssh_key.as_ref()).await?;
    let containers = session.list_containers().await?;

    Ok((
        StatusCode::OK,
        Json(ContainerListResponse { containers }),
    ))
}
