mod docker_local;
mod docker_remote;
mod remote;

pub use remote::connect_remote;

use remote::{remote_read, remote_resize, remote_write};

use crate::ShellError;

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
    DockerLocal,
    DockerRemote,
    Remote(russh::Channel<russh::client::Msg>),
}

impl Terminal {
    pub async fn write(&self, input: &[u8]) -> Result<(), ShellError> {
        match self {
            Terminal::DockerLocal => unimplemented!(),
            Terminal::DockerRemote => unimplemented!(),
            Terminal::Remote(channel) => remote_write(channel, input).await,
        }
    }

    pub async fn read(&mut self) -> Option<Vec<u8>> {
        match self {
            Terminal::DockerLocal => unimplemented!(),
            Terminal::DockerRemote => unimplemented!(),
            Terminal::Remote(channel) => remote_read(channel).await,
        }
    }

    pub async fn resize(&self, cols: u32, rows: u32) -> Result<(), ShellError> {
        match self {
            Terminal::DockerLocal => unimplemented!(),
            Terminal::DockerRemote => unimplemented!(),
            Terminal::Remote(channel) => remote_resize(channel, cols, rows).await,
        }
    }
}
