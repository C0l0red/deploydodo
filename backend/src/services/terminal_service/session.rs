use bollard::Docker;
use std::time::Duration;

use crate::error::AppError;
use crate::services::server_service::{Server, ServerType};
use crate::services::ssh_service::{SshKey, SshService};

use super::auth::build_ssh_auth;
use super::containers;
use super::exec;
use super::types::{ContainerInfo, TerminalOutput};

pub struct TerminalSession {
    docker: Docker,
    _tunnel: Option<dodosh::DockerTunnel>,
    _ssh_session: Option<dodosh::SshSession>,
}

impl TerminalSession {
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>, AppError> {
        containers::do_list_containers(&self.docker).await
    }

    pub async fn run_command(
        &self,
        container_id: &str,
        command: &str,
    ) -> Result<Vec<TerminalOutput>, AppError> {
        exec::do_run_command(&self.docker, container_id, command).await
    }
}

pub async fn resolve_ssh_key(
    server: &Server,
    ssh_service: &SshService,
) -> Result<Option<SshKey>, AppError> {
    if server.server_type == ServerType::Remote {
        let key_id = server
            .ssh_key_id
            .ok_or(AppError::Validation(
                "No SSH key for remote server".into(),
            ))?;
        let key = ssh_service.get_key_by_id(key_id).await?;
        Ok(Some(key))
    } else {
        Ok(None)
    }
}

pub async fn connect(
    server: &Server,
    ssh_key: Option<&SshKey>,
) -> Result<TerminalSession, AppError> {
    match server.server_type {
        ServerType::Local => connect_local().await,
        ServerType::Remote => connect_remote(server, ssh_key).await,
    }
}

async fn connect_local() -> Result<TerminalSession, AppError> {
    let docker = Docker::connect_with_local_defaults().map_err(|e| {
        tracing::error!(error = %e, "local docker connect failed");
        AppError::LocalDockerConnect(e.to_string())
    })?;
    Ok(TerminalSession {
        docker,
        _tunnel: None,
        _ssh_session: None,
    })
}

async fn connect_remote(
    server: &Server,
    ssh_key: Option<&SshKey>,
) -> Result<TerminalSession, AppError> {
    let key = ssh_key.ok_or(AppError::Validation(
        "SSH key required for remote servers".into(),
    ))?;

    let auth = build_ssh_auth(key)?;
    let session = dodosh::SshSession::connect(
        &server.hostname,
        server.ssh_port.unwrap_or(22),
        &key.username,
        auth,
        Some(Duration::from_secs(30)),
    )
    .await
    .map_err(|e| AppError::RemoteDockerConnect(format!("SSH connection failed: {e}")))?;

    let tunnel = session
        .forward_docker_socket()
        .await
        .map_err(|e| AppError::RemoteDockerConnect(format!("Docker socket forward failed: {e}")))?;

    let docker = Docker::connect_with_http(
        &format!("localhost:{}", tunnel.local_port),
        10,
        bollard::API_DEFAULT_VERSION,
    )
    .map_err(|e| AppError::RemoteDockerConnect(format!("bollard connect over tunnel failed: {e}")))?;

    tracing::info!(
        port = tunnel.local_port,
        host = %server.hostname,
        "connected to remote docker via ssh tunnel"
    );

    Ok(TerminalSession {
        docker,
        _tunnel: Some(tunnel),
        _ssh_session: Some(session),
    })
}
