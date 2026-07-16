use std::{pin::Pin, time::Duration};

use bollard::{
    container::LogOutput,
    exec::{CreateExecOptions, ResizeExecOptions, StartExecOptions, StartExecResults},
    Docker,
};
use futures::Stream;
use tokio::{io::AsyncWrite, sync::Mutex, time};

use crate::{
    terminal::{TermSize, Terminal},
    DockerTunnel, ShellError, SshAuth, SshSession, SshTimeout,
};

type DockerInput = Pin<Box<dyn AsyncWrite + Send>>;
type DockerOutput = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;

pub struct DockerChannel {
    docker: bollard::Docker,
    exec_id: String,
    input: Mutex<DockerInput>,
    output: Mutex<DockerOutput>,
    _tunnel: Option<DockerTunnel>,
}

impl DockerChannel {
    pub async fn write(&self, input: &[u8]) -> Result<(), ShellError> {
        let input_mut = &mut self.input.lock().await;
        Ok(tokio::io::AsyncWriteExt::write_all(&mut input_mut.as_mut(), input).await?)
    }

    pub async fn read(&self) -> Option<Vec<u8>> {
        let output_mut = &mut self.output.lock().await;
        match futures_util::StreamExt::next(&mut output_mut.as_mut()).await {
            Some(Ok(log)) => Some(log.into_bytes().to_vec()),
            Some(Err(err)) => Some(
                format!("\r\n[session error: {err}]\r\n")
                    .into_bytes()
                    .to_vec(),
            ),
            _ => None,
        }
    }

    pub async fn resize(&self, cols: u32, rows: u32) -> Result<(), ShellError> {
        Ok(self
            .docker
            .resize_exec(
                &self.exec_id,
                ResizeExecOptions {
                    width: cols as u16,
                    height: rows as u16,
                },
            )
            .await?)
    }
}

pub async fn connect_docker_local(
    container_name: &str,
    size: TermSize,
    timeout_config: SshTimeout,
) -> Result<Terminal, ShellError> {
    let docker = Docker::connect_with_local_defaults()?;

    connect_docker(
        docker,
        container_name,
        size,
        timeout_config.connect_timeout_secs,
        None,
    )
    .await
    .map(Terminal::Docker)
}

pub async fn connect_docker_remote(
    hostname: &str,
    port: u16,
    username: &str,
    auth: SshAuth<'_>,
    container_name: &str,
    size: TermSize,
    timeout_config: SshTimeout,
) -> Result<Terminal, ShellError> {
    let session =
        SshSession::connect(hostname, port, username, auth, timeout_config.clone()).await?;

    let tunnel = session.forward_docker_socket().await?;

    let docker_proxy = format!("127.0.0.1:{}", tunnel.local_port);
    let docker = Docker::connect_with_http(
        &docker_proxy,
        timeout_config.connect_timeout_secs,
        bollard::API_DEFAULT_VERSION,
    )?;

    let channel = connect_docker(
        docker,
        container_name,
        size,
        timeout_config.connect_timeout_secs,
        Some(tunnel),
    )
    .await?;

    Ok(Terminal::Docker(channel))
}

async fn connect_docker(
    docker: Docker,
    container_name: &str,
    size: TermSize,
    connect_timeout: u64,
    tunnel: Option<DockerTunnel>,
) -> Result<DockerChannel, ShellError> {
    let request_timeout = Duration::from_secs(connect_timeout);
    let exec = time::timeout(
        request_timeout,
        docker.create_exec(container_name, get_exec_options()),
    )
    .await??;

    let StartExecResults::Attached { output, input } = time::timeout(
        request_timeout,
        docker.start_exec(
            &exec.id,
            Some(StartExecOptions {
                tty: true,
                ..Default::default()
            }),
        ),
    )
    .await??
    else {
        return Err(ShellError::AuthFailed);
    };

    docker
        .resize_exec(
            &exec.id,
            ResizeExecOptions {
                width: size.cols as u16,
                height: size.rows as u16,
            },
        )
        .await?;

    Ok(DockerChannel {
        docker,
        exec_id: exec.id,
        input: Mutex::new(input),
        output: Mutex::new(output),
        _tunnel: tunnel,
    })
}

fn get_exec_options() -> CreateExecOptions<String> {
    CreateExecOptions {
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        tty: Some(true),
        cmd: Some(vec!["/bin/sh".to_string()]),
        ..Default::default()
    }
}
