mod docker;
mod host;

pub use docker::{connect_docker_local, connect_docker_remote};
pub use host::connect_host;

use host::{host_read, host_resize, host_write};

use crate::{terminal::docker::DockerChannel, ShellError};

pub struct TermSize {
    cols: u32,
    rows: u32,
}

impl TermSize {
    pub fn dims(cols: u32, rows: u32) -> Self {
        Self { cols, rows }
    }
}

pub enum Terminal {
    Docker(DockerChannel),
    Host(russh::Channel<russh::client::Msg>),
}

impl Terminal {
    pub async fn write(&self, input: &[u8]) -> Result<(), ShellError> {
        match self {
            Terminal::Docker(channel) => channel.write(input).await,
            Terminal::Host(channel) => host_write(channel, input).await,
        }
    }

    pub async fn read(&mut self) -> Option<Vec<u8>> {
        match self {
            Terminal::Docker(channel) => channel.read().await,
            Terminal::Host(channel) => host_read(channel).await,
        }
    }

    pub async fn resize(&self, cols: u32, rows: u32) -> Result<(), ShellError> {
        match self {
            Terminal::Docker(channel) => channel.resize(cols, rows).await,
            Terminal::Host(channel) => host_resize(channel, cols, rows).await,
        }
    }
}
