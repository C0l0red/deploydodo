use thiserror::Error;
use tokio::time;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("ssh error: {0}")]
    Ssh(#[from] russh::Error),
    #[error("docker error: {0}")]
    Docker(#[from] bollard::errors::Error),
    #[error("timeout error: {0}")]
    Timeout(#[from] time::error::Elapsed),
    #[error("key error: {0}")]
    Key(#[from] russh::keys::Error),
    #[error("authentication failed")]
    AuthFailed,
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
}
