use std::sync::Arc;
use std::time::Duration;

use russh::client;
use russh::keys::ssh_key::PublicKey;
use russh::keys::{decode_secret_key, PrivateKeyWithHashAlg};

use super::error::ShellError;
use super::types::{CommandOutput, DockerStatus, SshAuth};

pub struct Handler;

impl client::Handler for Handler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct SshSession {
    handle: Arc<client::Handle<Handler>>,
}

#[derive(Clone)]
pub struct SshTimeout {
    inactivity_secs: Option<u64>,
    keepalive_secs: Option<u64>,
    pub(crate) connect_timeout_secs: u64,
    keepalive_max: usize,
}

#[derive(Default)]
pub struct SshTimeoutBuilder {
    inactivity_secs: Option<u64>,
    keepalive_secs: Option<u64>,
    connect_timeout_secs: Option<u64>,
    keepalive_max: Option<usize>,
}

impl SshTimeout {
    pub fn builder() -> SshTimeoutBuilder {
        SshTimeoutBuilder::default()
    }

    pub fn inactivity_secs(secs: u64) -> Self {
        Self::builder().inactivity_secs(secs).build()
    }
}

impl SshTimeoutBuilder {
    pub fn inactivity_secs(&mut self, secs: u64) -> &mut Self {
        self.inactivity_secs = Some(secs);
        self
    }

    pub fn keepalive_secs(&mut self, secs: u64) -> &mut Self {
        self.keepalive_secs = Some(secs);
        self
    }

    pub fn connect_timeout_secs(&mut self, secs: u64) -> &mut Self {
        self.connect_timeout_secs = Some(secs);
        self
    }

    pub fn build(&self) -> SshTimeout {
        SshTimeout {
            inactivity_secs: self.inactivity_secs,
            keepalive_secs: self.keepalive_secs,
            connect_timeout_secs: self.connect_timeout_secs.unwrap_or(10),
            keepalive_max: self.keepalive_max.unwrap_or(5),
        }
    }
}

impl SshSession {
    pub(crate) fn new(handle: client::Handle<Handler>) -> Self {
        Self {
            handle: Arc::new(handle),
        }
    }

    pub async fn connect(
        hostname: &str,
        port: u16,
        username: &str,
        auth: SshAuth<'_>,
        timeout_config: SshTimeout,
    ) -> Result<Self, ShellError> {
        session_init(hostname, port, username, auth, timeout_config).await
    }

    pub async fn disconnect(&self) -> Result<(), ShellError> {
        self.handle
            .disconnect(
                russh::Disconnect::ByApplication,
                "SSH connection closed",
                "English",
            )
            .await
            .map_err(ShellError::Ssh)
    }

    pub async fn check_root_access(&self) -> Result<bool, ShellError> {
        let id_output = self.run_command("id -u").await?;

        if id_output.exit_code == 0 && id_output.stdout.trim() == "0" {
            return Ok(true);
        }

        let sudo_output = self.run_command("sudo -n true").await?;

        Ok(sudo_output.exit_code == 0)
    }

    pub async fn is_docker_installed_via_snap(&self) -> Result<bool, ShellError> {
        let which = self.run_command("which docker").await?;
        if which.exit_code == 0 && which.stdout.contains("/snap/") {
            return Ok(true);
        }

        let snap = self.run_command("snap list docker").await?;
        Ok(snap.exit_code == 0)
    }

    pub async fn check_docker(&self) -> Result<DockerStatus, ShellError> {
        let which = self.run_command("command -v docker").await?;
        if which.exit_code != 0 {
            return Ok(DockerStatus {
                is_installed: false,
                is_running: false,
            });
        }

        let info = self.run_command("docker info").await?;
        Ok(DockerStatus {
            is_installed: true,
            is_running: info.exit_code == 0,
        })
    }

    pub async fn open_channel(&self) -> Result<russh::Channel<russh::client::Msg>, ShellError> {
        Ok(self.handle.channel_open_session().await?)
    }

    pub async fn run_command(&self, command: &str) -> Result<CommandOutput, ShellError> {
        let mut channel = self.open_channel().await?;
        channel.exec(true, command).await?;

        let mut stdout = String::new();
        let mut exit_code = 0u32;

        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                russh::ChannelMsg::Data { data } => {
                    stdout.push_str(&String::from_utf8_lossy(&data));
                }
                russh::ChannelMsg::ExitStatus { exit_status } => {
                    exit_code = exit_status;
                }
                _ => {}
            }
        }
        Ok(CommandOutput { stdout, exit_code })
    }

    pub async fn forward_docker_socket(&self) -> Result<crate::DockerTunnel, ShellError> {
        crate::forward_docker_socket(self.handle.clone()).await
    }
}

async fn session_init(
    hostname: &str,
    port: u16,
    username: &str,
    auth: SshAuth<'_>,
    timeout_config: SshTimeout,
) -> Result<SshSession, ShellError> {
    let config = Arc::new(client::Config {
        inactivity_timeout: timeout_config.inactivity_secs.map(Duration::from_secs),
        keepalive_interval: timeout_config.keepalive_secs.map(Duration::from_secs),
        keepalive_max: timeout_config.keepalive_max,
        ..Default::default()
    });

    let mut handle = client::connect(config, (hostname, port), Handler).await?;

    let authenticated = match auth {
        SshAuth::Password(password) => handle.authenticate_password(username, password).await?,
        SshAuth::Key {
            private_key,
            passphrase,
        } => {
            let key = decode_secret_key(private_key, passphrase)?;
            let hash = handle.best_supported_rsa_hash().await?.flatten();
            handle
                .authenticate_publickey(username, PrivateKeyWithHashAlg::new(Arc::new(key), hash))
                .await?
        }
    };

    if !authenticated.success() {
        return Err(ShellError::AuthFailed);
    }

    Ok(SshSession::new(handle))
}
