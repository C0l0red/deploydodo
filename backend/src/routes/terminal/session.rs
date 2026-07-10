use dodosh::{terminal, SshTimeout};

use crate::dependencies::Dependencies;
use crate::error::{AppError, AppResult};
use crate::routes::terminal::TerminalParams;

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

    Ok(terminal::connect_remote(
        server.hostname(),
        *server.ssh_port(),
        ssh_key.username(),
        (&ssh_key).into(),
        params.into(),
        SshTimeout::keepalive_secs(30),
    )
    .await?)
}
