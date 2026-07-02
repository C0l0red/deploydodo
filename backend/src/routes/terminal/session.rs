use crate::dependencies::Dependencies;
use crate::error::AppError;
use crate::services::terminal_service::{connect, resolve_ssh_key, TerminalSession};

pub async fn establish_terminal_session(
    deps: &Dependencies,
    server_id: i64,
    token: &str,
) -> Result<TerminalSession, AppError> {
    let valid = deps.session_service.validate_session(token.trim()).await?;
    if !valid {
        return Err(AppError::Unauthorized);
    }

    let server = deps.server_service.get_server_by_id(server_id).await?;
    let ssh_key = resolve_ssh_key(&server, &deps.ssh_service).await?;

    connect(&server, ssh_key.as_ref()).await
}
