use dodosh::terminal::TermSize;
use dodosh::{terminal, SshTimeout};

use crate::dependencies::Dependencies;
use crate::error::AppResult;
use crate::routes::terminal::TerminalParams;
use crate::services::server_service::ServerId;

pub async fn terminal_init(
    server_id: ServerId,
    params: TerminalParams,
    deps: &Dependencies,
) -> AppResult<terminal::Terminal> {
    let server = deps.server_service.get_server_by_id(server_id).await?;

    let ssh_key = deps.ssh_service.get_key_for_server(&server).await?;

    let timeout_config = SshTimeout::builder()
        .inactivity_secs(60)
        .keepalive_secs(30)
        .build();

    if let TerminalParams {
        cols,
        rows,
        container_name: Some(ref container_name),
    } = params
    {
        let server_type = server.server_type();

        let params = TermSize::dims(cols, rows);

        if server_type.is_local() {
            Ok(terminal::connect_docker_local(container_name, params, timeout_config).await?)
        } else {
            Ok(terminal::connect_docker_remote(
                server.hostname(),
                *server.ssh_port(),
                ssh_key.username(),
                ssh_key.into(),
                container_name,
                params,
                timeout_config,
            )
            .await?)
        }
    } else {
        Ok(terminal::connect_host(
            server.hostname(),
            *server.ssh_port(),
            ssh_key.username(),
            ssh_key.into(),
            params.into(),
            timeout_config,
        )
        .await?)
    }
}
