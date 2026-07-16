use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{
    env::get_env,
    error::{AppError, AppResult},
    services::server_service::Server,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "authtype", rename_all = "lowercase")]
pub enum AuthType {
    Password,
    KeyPair,
}

impl AuthType {
    pub fn is_keypair(&self) -> bool {
        matches!(self, Self::KeyPair)
    }
}

pub enum SshKey {
    Password {
        id: i64,
        name: String,
        username: String,
        password: String,
    },
    KeyPair {
        id: i64,
        name: String,
        username: String,
        private_key: String,
        public_key: Option<String>,
    },
}

impl<'a> SshKey {
    pub fn id(&'a self) -> &'a i64 {
        match self {
            SshKey::Password { id, .. } | SshKey::KeyPair { id, .. } => id,
        }
    }

    pub fn username(&'a self) -> &'a str {
        match self {
            SshKey::Password { username, .. } | SshKey::KeyPair { username, .. } => username,
        }
    }
}

impl<'a> From<&'a SshKey> for dodosh::SshAuth<'a> {
    fn from(value: &'a SshKey) -> Self {
        match value {
            SshKey::KeyPair { private_key, .. } => Self::Key {
                private_key,
                passphrase: None,
            },
            SshKey::Password { password, .. } => Self::Password(password),
        }
    }
}

pub struct SshService {
    db: Arc<PgPool>,
    host_ssh_username: String,
    host_ssh_private_key: String,
}

impl SshService {
    pub fn new(db: Arc<PgPool>) -> Self {
        let env = get_env();

        Self {
            db,
            host_ssh_username: env.local_ssh_username.to_owned(),
            host_ssh_private_key: env.local_ssh_private_key.to_owned(),
        }
    }

    pub async fn create_password_auth(
        &self,
        name: &str,
        username: &str,
        password: &str,
    ) -> AppResult<SshKey> {
        let id: i64 = sqlx::query_scalar!(
            "INSERT INTO ssh_keys (name, username, password, auth_type, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            name,
            username,
            password,
            AuthType::Password as _,
            Utc::now()
        )
        .fetch_one(&*self.db)
        .await
        ?;

        Ok(SshKey::Password {
            id,
            name: name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        })
    }

    pub async fn create_key_auth(
        &self,
        name: &str,
        username: &str,
        private_key: &str,
        public_key: Option<&str>,
    ) -> AppResult<SshKey> {
        let id: i64 = sqlx::query_scalar!(
            "INSERT INTO ssh_keys (name, username, private_key, public_key, auth_type, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            name,
            username,
            private_key,
            public_key,
            AuthType::KeyPair as _,
            Utc::now()
        )
        .fetch_one(&*self.db)
        .await
        ?;

        Ok(SshKey::KeyPair {
            id,
            name: name.to_string(),
            username: username.to_string(),
            private_key: private_key.to_string(),
            public_key: public_key.map(|key| key.to_string()),
        })
    }

    pub async fn get_key_by_id(&self, key_id: &i64) -> AppResult<SshKey> {
        let row = sqlx::query!(
            r#"SELECT id, name, username, auth_type AS "auth_type: AuthType", password, private_key, public_key FROM ssh_keys WHERE id = $1"#,
            key_id
        )
        .fetch_optional(&*self.db)
        .await
        ?;

        let row = row.ok_or(AppError::Validation("SSH key not found".into()))?;

        Ok(if row.auth_type.is_keypair() {
            SshKey::KeyPair {
                id: row.id,
                name: row.name,
                username: row.username,
                private_key: row
                    .private_key
                    .ok_or(AppError::validation("private_key is None in Server"))?,
                public_key: row.public_key,
            }
        } else {
            SshKey::Password {
                id: row.id,
                name: row.name,
                username: row.username,
                password: row
                    .password
                    .ok_or(AppError::validation("password is None in Server"))?,
            }
        })
    }

    pub async fn get_key_for_server(&self, server: &Server) -> AppResult<SshKey> {
        match server {
            Server::Local { .. } => Ok(SshKey::KeyPair {
                id: 0,
                name: "local-server".to_string(),
                username: self.host_ssh_username.clone(),
                private_key: self.host_ssh_private_key.clone(),
                public_key: None,
            }),
            Server::Remote { ssh_key_id, .. } => self.get_key_by_id(ssh_key_id).await,
        }
    }
}
