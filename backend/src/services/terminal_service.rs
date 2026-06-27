use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::Docker;
use dodosh::{DockerTunnel, SshAuth, SshSession};
use futures_util::StreamExt;
use std::time::Duration;

use crate::error::AppError;
use crate::services::server_service::{Server, ServerType};
use crate::services::ssh_service::SshKey;

pub struct TerminalSession {
    pub docker: Docker,
    _tunnel: Option<DockerTunnel>,
    _ssh_session: Option<SshSession>,
}

pub struct TerminalService;

impl TerminalService {
    pub fn new() -> Self {
        Self
    }

    pub async fn connect(
        server: &Server,
        ssh_key: Option<&SshKey>,
    ) -> Result<TerminalSession, AppError> {
        match server.server_type {
            ServerType::Local => {
                let docker = Docker::connect_with_local_defaults().map_err(
                    |e| {
                        tracing::error!(error = %e, "local docker connect failed");
                        AppError::DockerConnection(e.to_string())
                    },
                )?;
                Ok(TerminalSession {
                    docker,
                    _tunnel: None,
                    _ssh_session: None,
                })
            }
            ServerType::Remote => {
                let key = ssh_key.ok_or(AppError::Validation(
                    "SSH key required for remote servers".into(),
                ))?;

                let auth = build_ssh_auth(key)?;
                let session = SshSession::connect(
                    &server.hostname,
                    server.ssh_port.unwrap_or(22),
                    &key.username,
                    auth,
                    Some(Duration::from_secs(30)),
                )
                .await
                .map_err(|e| {
                    AppError::DockerConnection(format!(
                        "SSH connection failed: {e}"
                    ))
                })?;

                let tunnel = session.forward_docker_socket().await.map_err(
                    |e| {
                        AppError::DockerConnection(format!(
                            "Docker socket forward failed: {e}"
                        ))
                    },
                )?;

                let docker = Docker::connect_with_http(
                    &format!("localhost:{}", tunnel.local_port),
                    10,
                    bollard::API_DEFAULT_VERSION,
                )
                .map_err(|e| {
                    AppError::DockerConnection(format!(
                        "bollard connect over tunnel failed: {e}"
                    ))
                })?;

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
        }
    }

    pub async fn list_containers(
        docker: &Docker,
    ) -> Result<Vec<ContainerInfo>, AppError> {
        let options = bollard::query_parameters::ListContainersOptions {
            all: false,
            ..Default::default()
        };

        let containers = docker
            .list_containers(Some(options))
            .await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;

        Ok(containers
            .into_iter()
            .map(|c| ContainerInfo {
                id: c.id.unwrap_or_default(),
                name: c
                    .names
                    .unwrap_or_default()
                    .first()
                    .cloned()
                    .unwrap_or_default(),
                image: c.image.unwrap_or_default(),
            })
            .collect())
    }

    pub async fn run_command(
        docker: &Docker,
        container_id: &str,
        command: &str,
    ) -> Result<Vec<TerminalOutput>, AppError> {
        let exec = docker
            .create_exec(
                container_id,
                CreateExecOptions {
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    cmd: Some(vec![
                        "sh".into(),
                        "-c".into(),
                        command.to_string(),
                    ]),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;

        let result = docker
            .start_exec(&exec.id, None::<StartExecOptions>)
            .await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;

        let mut output = Vec::new();
        if let StartExecResults::Attached {
            output: mut stream, ..
        } = result
        {
            while let Some(item) = stream.next().await {
                match item {
                    Ok(bollard::container::LogOutput::StdOut { message })
                    | Ok(bollard::container::LogOutput::StdErr { message })
                    | Ok(
                        bollard::container::LogOutput::Console { message },
                    ) => {
                        output.push(TerminalOutput::Stdout(
                            String::from_utf8_lossy(&message).to_string(),
                        ));
                    }
                    Ok(_) => {}
                    Err(e) => {
                        return Err(AppError::DockerConnection(
                            e.to_string(),
                        ));
                    }
                }
            }
        }

        Ok(output)
    }
}

fn build_ssh_auth(key: &SshKey) -> Result<SshAuth<'_>, AppError> {
    match key.auth_type {
        crate::services::types::AuthType::Password => {
            let pass = key.password.as_deref().ok_or(
                AppError::MissingKeySecret,
            )?;
            Ok(SshAuth::Password(pass))
        }
        crate::services::types::AuthType::KeyPair => {
            let private_key = key.private_key.as_deref().ok_or(
                AppError::MissingKeySecret,
            )?;
            Ok(SshAuth::Key {
                private_key,
                passphrase: None,
            })
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone)]
pub enum TerminalOutput {
    Stdout(String),
    Stderr(String),
}
