#[derive(Debug, Clone)]
pub enum SshAuth<'a> {
    Password(&'a str),
    Key {
        private_key: &'a str,
        passphrase: Option<&'a str>,
    },
}

#[derive(Debug, Clone)]
pub struct DockerStatus {
    pub is_installed: bool,
    pub is_running: bool,
}

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub exit_code: u32,
}
