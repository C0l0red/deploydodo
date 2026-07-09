use std::sync::Arc;

use russh::client;

use super::error::ShellError;
use super::session::Handler;

pub struct DockerTunnel {
    pub local_port: u16,
    _task: tokio::task::JoinHandle<()>,
}

impl Drop for DockerTunnel {
    fn drop(&mut self) {
        self._task.abort();
    }
}

pub async fn forward_docker_socket(
    handle: Arc<client::Handle<Handler>>,
) -> Result<DockerTunnel, ShellError> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let local_port = listener.local_addr()?.port();

    let task = tokio::spawn(async move {
        while let Ok((tcp, _)) = listener.accept().await {
            let h = handle.clone();
            tokio::spawn(async move {
                if let Err(e) = forward_single(tcp, h).await {
                    let _ = e;
                }
            });
        }
    });

    Ok(DockerTunnel {
        local_port,
        _task: task,
    })
}

async fn forward_single(
    tcp: tokio::net::TcpStream,
    handle: Arc<client::Handle<Handler>>,
) -> Result<(), ShellError> {
    let mut channel = handle
        .channel_open_direct_streamlocal("/var/run/docker.sock")
        .await?;

    let channel_id = channel.id();
    let (mut tcp_rx, mut tcp_tx) = tcp.into_split();

    let h_send = handle.clone();
    let tcp_to_ssh = tokio::spawn(async move {
        let mut buf = vec![0u8; 32768];
        loop {
            match tokio::io::AsyncReadExt::read(&mut tcp_rx, &mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    if h_send
                        .data(channel_id, buf[..n].to_vec())
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    let ssh_to_tcp = tokio::spawn(async move {
        loop {
            match channel.wait().await {
                Some(russh::ChannelMsg::Data { data }) => {
                    if tokio::io::AsyncWriteExt::write_all(&mut tcp_tx, &data)
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Some(russh::ChannelMsg::Close) | None => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = tcp_to_ssh => {},
        _ = ssh_to_tcp => {},
    }

    Ok(())
}
