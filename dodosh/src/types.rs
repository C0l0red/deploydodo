pub enum SshAuth {
    Password(String),
    Key {
        private_key: String,
        passphrase: Option<String>,
    },
}

pub struct DockerStatus {
    pub is_installed: bool,
    pub is_running: bool,
}

pub struct CommandOutput {
    pub stdout: String,
    pub exit_code: u32,
}
