use crate::error::AppError;
use crate::services::ssh_service::SshKey;
use crate::services::types::AuthType;

pub fn build_ssh_auth(key: &SshKey) -> Result<dodosh::SshAuth<'_>, AppError> {
    match key.auth_type {
        AuthType::Password => {
            let pass = key
                .password
                .as_deref()
                .ok_or(AppError::MissingKeySecret)?;
            Ok(dodosh::SshAuth::Password(pass))
        }
        AuthType::KeyPair => {
            let private_key = key
                .private_key
                .as_deref()
                .ok_or(AppError::MissingKeySecret)?;
            Ok(dodosh::SshAuth::Key {
                private_key,
                passphrase: None,
            })
        }
    }
}
