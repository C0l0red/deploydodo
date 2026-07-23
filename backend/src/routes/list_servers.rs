use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::dependencies::Dependencies;
use crate::error::AppResult;
use crate::extractors::Auth;
use crate::new_types::ServerPort;
use crate::services::server_service::ServerId;
use crate::services::types;

#[derive(Serialize, ToSchema)]
pub struct ServerResponse {
    pub id: ServerId,
    pub name: String,
    #[serde(rename = "serverType")]
    pub server_type: types::ServerType,
    pub hostname: String,
    #[serde(rename = "sshPort")]
    pub ssh_port: ServerPort,
}

#[utoipa::path(
    get,
    path = "/api/servers",
    params(
        ("Authorization" = String, Header, description = "authorization token")
    ),
    responses(
        (status = 200, description = "List of all servers", body = Vec<ServerResponse>),
    ),
    tag = "servers"
)]
pub async fn list_servers(
    _: Auth, //FIXME: This is a code smell, I've made sure the compiler flags it so we don't forget
    State(deps): State<Dependencies>,
) -> AppResult<(StatusCode, Json<Vec<ServerResponse>>)> {
    let servers = deps.server_service.list_servers().await?;

    Ok((
        StatusCode::OK,
        Json(
            servers
                .into_iter()
                .map(|s| ServerResponse {
                    id: s.id(),
                    name: s.name().to_owned(),
                    server_type: s.server_type().to_owned(),
                    hostname: s.hostname().to_owned(),
                    ssh_port: s.ssh_port(),
                })
                .collect(),
        ),
    ))
}
