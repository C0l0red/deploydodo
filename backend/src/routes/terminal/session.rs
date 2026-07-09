use dodosh::{terminal, SshTimeout};

use crate::dependencies::Dependencies;
use crate::error::{AppError, AppResult};
use crate::routes::terminal::TerminalParams;
use crate::services::server_service;
use crate::services::ssh_service::SshKey;
use crate::services::types::ServerType;

pub async fn terminal_init(
    server_id: i64,
    params: TerminalParams,
    deps: &Dependencies,
) -> AppResult<terminal::Terminal> {
    let valid = deps
        .session_service
        .validate_session(params.token.trim())
        .await?;
    if !valid {
        return Err(AppError::Unauthorized);
    }

    let server = deps.server_service.get_server_by_id(server_id).await?;
    let ssh_key = deps.ssh_service.get_key_for_server(&server).await?;

    match server.server_type {
        ServerType::Local => local_terminal_init(),
        ServerType::Remote => remote_terminal_init(server, params, ssh_key).await,
    }
}

async fn remote_terminal_init(
    server: server_service::Server,
    params: TerminalParams,
    ssh_key: SshKey,
) -> AppResult<terminal::Terminal> {
    Ok(terminal::connect_remote(
        &server.hostname,
        server.ssh_port()?,
        ssh_key.username(),
        (&ssh_key).try_into()?,
        params.into(),
        SshTimeout::keepalive_secs(30),
    )
    .await?)
}

fn local_terminal_init() -> AppResult<terminal::Terminal> {
    Err(AppError::InternalServerError(
        "local terminal could not be initialized".to_string(),
    ))
}
