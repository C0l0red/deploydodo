use std::time::Duration;
use tokio::time;

use crate::{
    session::SshTimeout,
    terminal::{self, TermSize},
    ShellError, SshAuth, SshSession,
};

pub async fn connect_host(
    hostname: String,
    port: u16,
    username: String,
    auth: SshAuth,
    size: TermSize,
    timeout_config: SshTimeout,
) -> Result<terminal::Terminal, ShellError> {
    let session =
        SshSession::connect(hostname, port, username, auth, timeout_config.clone()).await?;

    let channel = session.open_channel().await?;

    let request_timeout = Duration::from_secs(timeout_config.connect_timeout_secs);

    time::timeout(
        request_timeout,
        channel.request_pty(true, "xterm-256color", size.cols, size.rows, 0, 0, &[]),
    )
    .await??;
    time::timeout(request_timeout, channel.request_shell(true)).await??;

    Ok(terminal::Terminal::Host(channel))
}

pub async fn host_write(
    channel: &russh::Channel<russh::client::Msg>,
    input: &[u8],
) -> Result<(), ShellError> {
    Ok(channel.data(input).await?)
}

pub async fn host_read(channel: &mut russh::Channel<russh::client::Msg>) -> Option<Vec<u8>> {
    loop {
        match channel.wait().await? {
            russh::ChannelMsg::Data { data } => return Some(data.to_vec()),
            russh::ChannelMsg::ExitStatus { .. } => return None,
            _ => continue,
        }
    }
}

pub async fn host_resize(
    channel: &russh::Channel<russh::client::Msg>,
    cols: u32,
    rows: u32,
) -> Result<(), ShellError> {
    Ok(channel.window_change(cols, rows, 0, 0).await?)
}
