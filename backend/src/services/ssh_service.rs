use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use utoipa::ToSchema;

use crate::{
    error::{AppError, AppResult},
    services::{server_service::Server, types::ServerType},
};

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum AuthType {
    Password,
    KeyPair,
}

pub struct SshKey {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub auth_type: AuthType,
    pub password: Option<String>,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

#[allow(dead_code)]
impl SshKey {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn get_secret(&self) -> Result<&str, AppError> {
        match self.auth_type {
            AuthType::Password => self.password.as_deref().ok_or(AppError::MissingKeySecret),
            AuthType::KeyPair => self
                .private_key
                .as_deref()
                .ok_or(AppError::MissingKeySecret),
        }
    }
}

impl<'a> TryFrom<&'a SshKey> for dodosh::SshAuth<'a> {
    type Error = AppError;

    fn try_from(value: &'a SshKey) -> AppResult<Self> {
        match value.auth_type {
            AuthType::KeyPair => Ok(Self::Key {
                private_key: value.get_secret()?,
                passphrase: None,
            }),
            AuthType::Password => Ok(Self::Password(value.get_secret()?)),
        }
    }
}

pub struct SshService {
    db: Arc<SqlitePool>,
    host_ssh_username: String,
    host_ssh_private_key: String,
}

impl SshService {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        let host_ssh_username = std::env::var("HOST_SSH_USERNAME")
            .expect("The variable HOST_SSH_USERNAME must be present at runtime");

        let host_ssh_private_key = std::env::var("HOST_SSH_PRIVATE_KEY")
            .ok()
            .map(|path| {
                std::fs::read_to_string(path)
                    .expect("The path stored in HOST_SSH_PRIVATE_KEY does not exist")
            })
            .expect("The variable HOST_SSH_PRIVATE_KEY must be present at runtime");

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
    ) -> Result<SshKey, AppError> {
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
        .map_err(AppError::Database)?;

        Ok(SshKey {
            id,
            name: name.to_string(),
            username: username.to_string(),
            password: Some(password.to_string()),
            auth_type: AuthType::Password,
            private_key: None,
            public_key: None,
        })
    }

    pub async fn create_key_auth(
        &self,
        name: &str,
        username: &str,
        private_key: &str,
        public_key: Option<&str>,
    ) -> Result<SshKey, AppError> {
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
        .map_err(AppError::Database)?;

        Ok(SshKey {
            id,
            name: name.to_string(),
            username: username.to_string(),
            password: None,
            auth_type: AuthType::KeyPair,
            private_key: Some(private_key.to_string()),
            public_key: public_key.map(|key| key.to_string()),
        })
    }

    pub async fn get_key_by_id(&self, key_id: i64) -> AppResult<SshKey> {
        let row = sqlx::query(
            "SELECT id, name, username, auth_type, password, private_key, public_key FROM ssh_keys WHERE id = $1",
        )
        .bind(key_id)
        .fetch_optional(&*self.db)
        .await
        .map_err(AppError::Database)?;

        let row = row.ok_or(AppError::Validation("SSH key not found".into()))?;

        Ok(SshKey {
            id: row.try_get("id").map_err(AppError::Database)?,
            name: row.try_get("name").map_err(AppError::Database)?,
            username: row.try_get("username").map_err(AppError::Database)?,
            auth_type: row.try_get("auth_type").map_err(AppError::Database)?,
            password: row.try_get("password").ok(),
            private_key: row.try_get("private_key").ok(),
            public_key: row.try_get("public_key").ok(),
        })
    }

    pub async fn get_key_for_server(&self, server: &Server) -> AppResult<SshKey> {
        match server.server_type {
            ServerType::Local => Ok(SshKey {
                id: 0,
                name: "local-server".to_string(),
                username: self.host_ssh_username.clone(),
                auth_type: AuthType::KeyPair,
                password: None,
                private_key: Some(self.host_ssh_private_key.clone()),
                public_key: None,
            }),
            ServerType::Remote => {
                let key_id = server.ssh_key_id.ok_or(AppError::InternalServerError(
                    "SSH key missing for remote server".into(),
                ))?;

                self.get_key_by_id(key_id).await
            }
        }
    }
}
