use thiserror::Error;

#[derive(Debug, Error)]
pub enum SshError {
    #[error("ssh error: {0}")]
    Ssh(#[from] russh::Error),
    #[error("key error: {0}")]
    Key(#[from] russh::keys::Error),
    #[error("authentication failed")]
    AuthFailed,
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
}
