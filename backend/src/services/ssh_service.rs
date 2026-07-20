use std::ops::Deref;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::new_types::{SshPrivateKey, SshPublicKey};
use crate::routes::create_remote_server::SshAuthRequest;
use crate::{entity, env::get_env, error::{AppError, AppResult}, impl_deref, services::server_service::Server};

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

#[derive(sqlx::Type, Debug)]
#[sqlx(transparent)]
pub struct SshKeyId(i64);

impl_deref!(SshKeyId, i64);

entity! {
    #[derive(sqlx::FromRow)]
    pub struct SshKeyRow {
        id: SshKeyId,
        name: String,
        username: String,
        password: Option<String>,
        private_key: Option<SshPrivateKey>,
        public_key: Option<SshPublicKey>,
        auth_type: AuthType,
        created_at: DateTime<Utc>,
    }
}

pub enum SshKey {
    Password {
        id: SshKeyId,
        name: String,
        username: String,
        password: String,
    },
    KeyPair {
        id: SshKeyId,
        name: String,
        username: String,
        private_key: SshPrivateKey,
        public_key: Option<SshPublicKey>,
    },
}

impl TryFrom<SshKeyRow> for SshKey {
    type Error = AppError;

    fn try_from(value: SshKeyRow) -> Result<Self, Self::Error> {
        match value.auth_type {
            AuthType::Password => Ok(Self::Password {
                id: value.id,
                name: value.name,
                username: value.username,
                password: value.password.ok_or(AppError::CouldNotParse(
                    "SshKey missing password".to_string(),
                ))?,
            }),
            AuthType::KeyPair => Ok(Self::KeyPair {
                id: value.id,
                name: value.name,
                username: value.username,
                private_key: value.private_key.ok_or(AppError::CouldNotParse(
                    "SshKey missing private key".to_string(),
                ))?,
                public_key: value.public_key,
            }),
        }
    }
}

impl From<SshKey> for SshKeyRow {
    fn from(value: SshKey) -> Self {
        match value {
            SshKey::Password {
                id,
                name,
                username,
                password,
            } => Self {
                id,
                name,
                username,
                password: Some(password),
                auth_type: AuthType::Password,
                created_at: Utc::now(),
                private_key: None,
                public_key: None,
            },
            SshKey::KeyPair {
                id,
                name,
                username,
                public_key,
                private_key,
            } => Self {
                id,
                name,
                username,
                public_key,
                private_key: Some(private_key),
                auth_type: AuthType::KeyPair,
                created_at: Utc::now(),
                password: None,
            },
        }
    }
}

impl NewSshKeyRow {
    pub fn new(key_name: String, ssh_auth_request: SshAuthRequest) -> Self {
        match ssh_auth_request {
            SshAuthRequest::Password { username, password } => Self {
                name: key_name,
                username: username.to_owned(),
                password: Some(password.to_owned()),
                private_key: None,
                public_key: None,
                auth_type: AuthType::Password,
                created_at: Utc::now(),
            },
            SshAuthRequest::KeyPair {
                username,
                public_key,
                private_key,
            } => Self {
                name: key_name,
                username: username.to_owned(),
                password: None,
                private_key: Some(private_key),
                public_key,
                auth_type: AuthType::KeyPair,
                created_at: Utc::now(),
            },
        }
    }
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
    
    pub async fn create_ssh_key(&self, new_ssh_key_row: NewSshKeyRow) -> AppResult<SshKey> {
        let ssh_key_row = sqlx::query_as!(
            SshKeyRow,
            r#"
            INSERT INTO ssh_keys (name, username, private_key, public_key, auth_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id AS "id!: _",
                name,
                username,
                password,
                private_key as "private_key: _",
                public_key as "public_key: _",
                auth_type as "auth_type!: _",
                created_at
            "#,
            new_ssh_key_row.name,
            new_ssh_key_row.username,
            new_ssh_key_row.private_key.as_ref().map(Deref::deref),
            new_ssh_key_row.public_key.as_ref().map(Deref::deref),
            new_ssh_key_row.auth_type as AuthType,
            new_ssh_key_row.created_at
        )
            .fetch_one(&*self.db)
            .await?;

        Ok(SshKey::try_from(ssh_key_row)?)
    }

    pub async fn get_key_by_id(&self, key_id: &SshKeyId) -> AppResult<SshKey> {
        let ssh_key_row = sqlx::query_as!(
            SshKeyRow,
            r#"
            SELECT 
                id AS "id!: _",
                name,
                username,
                password,
                private_key as "private_key: _",
                public_key as "public_key: _",
                auth_type as "auth_type!: _",
                created_at
            FROM ssh_keys WHERE id = $1
            "#,
            key_id.deref()
        )
        .fetch_optional(&*self.db)
        .await?
            .ok_or(AppError::Validation("SSH key not found".into()))?;
        
        Ok(SshKey::try_from(ssh_key_row)?)
    }

    pub async fn get_key_for_server(&self, server: &Server) -> AppResult<SshKey> {
        match server {
            Server::Local { .. } => Ok(SshKey::KeyPair {
                id: SshKeyId(0),
                name: "local-server".to_string(),
                username: self.host_ssh_username.clone(),
                private_key: SshPrivateKey::try_new(self.host_ssh_private_key.clone())?,
                public_key: None,
            }),
            Server::Remote { ssh_key_id, .. } => self.get_key_by_id(ssh_key_id).await,
        }
    }
}
