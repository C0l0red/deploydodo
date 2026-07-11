use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use utoipa::ToSchema;

use crate::{
    error::{AppError, AppResult},
    services::server_service::Server,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
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
        let host_ssh_username = std::env::var("LOCAL_SSH_USERNAME")
            .expect("The variable HOST_SSH_USERNAME must be present at runtime");

        let host_ssh_private_key = std::env::var("LOCAL_SSH_PRIVATE_KEY")
            .ok()
            .map(|path| {
                std::fs::read_to_string(path)
                    .expect("The path stored in LOCAL_SSH_PRIVATE_KEY does not exist")
            })
            .expect("The variable LOCAL_SSH_PRIVATE_KEY must be present at runtime");

        Self {
            db,
            host_ssh_username,
            host_ssh_private_key,
        }
    }

    pub async fn create_password_auth(
        &self,
        name: &str,
        username: &str,
        password: &str,
    ) -> AppResult<SshKey> {
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO ssh_keys (name, username, password, auth_type, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(name)
        .bind(username)
        .bind(password)
        .bind(AuthType::Password)
        .bind(Utc::now())
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
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO ssh_keys (name, username, private_key, public_key, auth_type, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        )
        .bind(name)
        .bind(username)
        .bind(private_key)
        .bind(public_key)
        .bind(AuthType::KeyPair)
        .bind(Utc::now())
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
        let row = sqlx::query(
            "SELECT id, name, username, auth_type, password, private_key, public_key FROM ssh_keys WHERE id = $1",
        )
        .bind(key_id)
        .fetch_optional(&*self.db)
        .await
        ?;

        let row = row.ok_or(AppError::Validation("SSH key not found".into()))?;

        let auth_type: AuthType = row.try_get("auth_type")?;

        Ok(if auth_type.is_keypair() {
            SshKey::KeyPair {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                username: row.try_get("username")?,
                private_key: row.try_get("private_key")?,
                public_key: row.try_get("public_key").ok(),
            }
        } else {
            SshKey::Password {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                username: row.try_get("username")?,
                password: row.try_get("password")?,
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
