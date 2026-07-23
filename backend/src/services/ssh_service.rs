use crate::impl_display_via_to_string;
use std::ops::Deref;
use std::sync::Arc;

use crate::new_types::{SshPrivateKey, SshPublicKey};
use crate::routes::create_remote_server::SshAuthRequest;
use crate::{
    entity, entity_id,
    env::get_env,
    error::{AppError, AppResult},
    impl_deref, impl_deserialize_via_try_new, newtype,
    services::server_service::Server,
};
use chrono::{DateTime, Utc};
use dodosh::SshAuth;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

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
                private_key: value.private_key.ok_or(AppError::CouldNotParse(format!(
                    "SshKey missing private key. SshKey ID: {}",
                    value.id
                )))?,
                public_key: value.public_key,
            }),
        }
    }
}

impl SshKeyRowInput {
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

impl SshKey {
    pub fn id(&self) -> SshKeyId {
        match self {
            SshKey::Password { id, .. } | SshKey::KeyPair { id, .. } => *id,
        }
    }

    pub fn username(&self) -> String {
        match self {
            SshKey::Password { username, .. } | SshKey::KeyPair { username, .. } => {
                username.to_owned()
            }
        }
    }
}

impl From<SshKey> for SshAuth {
    fn from(value: SshKey) -> Self {
        match value {
            SshKey::KeyPair { private_key, .. } => Self::Key {
                private_key: private_key.clone(),
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

    pub async fn create_ssh_key(&self, ssh_key_row_input: SshKeyRowInput) -> AppResult<SshKey> {
        let ssh_key_row = sqlx::query_as!(
            SshKeyRow,
            r#"
            INSERT INTO ssh_keys (name, username, password, private_key, public_key, auth_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
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
            ssh_key_row_input.name,
            ssh_key_row_input.username,
            ssh_key_row_input.password,
            ssh_key_row_input.private_key.as_ref().map(Deref::deref),
            ssh_key_row_input.public_key.as_ref().map(Deref::deref),
            ssh_key_row_input.auth_type as AuthType,
            ssh_key_row_input.created_at
        )
            .fetch_one(&*self.db)
            .await?;

        SshKey::try_from(ssh_key_row)
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

        SshKey::try_from(ssh_key_row)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::new_types::NonEmptyString;
    use crate::routes::create_remote_server::SshAuthRequest;
    use crate::services::server_service::{Server, ServerId};
    use std::sync::Arc;

    fn make_service(pool: &PgPool) -> SshService {
        SshService {
            db: Arc::new(pool.clone()),
            host_ssh_username: "tester".into(),
            host_ssh_private_key:
                "-----BEGIN OPENSSH PRIVATE KEY-----\n-----END OPENSSH PRIVATE KEY-----".into(),
        }
    }

    #[sqlx::test]
    async fn create_ssh_key_inserts_password_ssh_keys(pool: PgPool) {
        // Construct service without invoking env helpers
        let service = make_service(&pool);

        let req = SshAuthRequest::Password {
            username: NonEmptyString::try_new("alice").unwrap(),
            password: NonEmptyString::try_new("s3cretpass").unwrap(),
        };
        let row_input = SshKeyRowInput::new("pw-key".to_string(), req);

        let created = service.create_ssh_key(row_input).await.unwrap();

        match created {
            SshKey::Password {
                name,
                username,
                password,
                ..
            } => {
                assert_eq!(name, "pw-key");
                assert_eq!(username, "alice");
                assert_eq!(password, "s3cretpass");
            }
            _ => panic!("expected password variant"),
        }
    }

    #[sqlx::test]
    async fn create_ssh_key_inserts_keypair_ssh_keys(pool: PgPool) {
        let service = make_service(&pool);

        let pem = "-----BEGIN OPENSSH PRIVATE KEY-----\n-----END OPENSSH PRIVATE KEY-----";
        let req = SshAuthRequest::KeyPair {
            username: NonEmptyString::try_new("bob").unwrap(),
            private_key: SshPrivateKey::try_new(pem).unwrap(),
            public_key: Some(
                SshPublicKey::try_new("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCs").unwrap(),
            ),
        };
        let row_input = SshKeyRowInput::new("kp-key".to_string(), req);

        let created = service.create_ssh_key(row_input).await.unwrap();

        match created {
            SshKey::KeyPair {
                name,
                username,
                private_key,
                public_key,
                ..
            } => {
                assert_eq!(name, "kp-key");
                assert_eq!(username, "bob");
                assert!(private_key.as_str().starts_with("-----BEGIN"));
                assert!(public_key.is_some());
            }
            _ => panic!("expected keypair variant"),
        }
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("ssh_keys")))]
    async fn get_key_by_id_returns_existing_ssh_key(pool: PgPool) {
        let service = make_service(&pool);

        // Look up the id of the seeded key by name
        let id: i64 = sqlx::query_scalar!("SELECT id FROM ssh_keys WHERE name = $1", "pw-fixture")
            .fetch_one(&pool)
            .await
            .unwrap();
        let key = service.get_key_by_id(&SshKeyId(id)).await.unwrap();

        match key {
            SshKey::Password {
                name,
                username,
                password,
                ..
            } => {
                assert_eq!(name, "pw-fixture");
                assert_eq!(username, "ada");
                assert_eq!(password, "hunter2");
            }
            _ => panic!("expected password variant"),
        }
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("ssh_keys")))]
    async fn get_key_by_id_returns_error_for_unknown_id(pool: PgPool) {
        let service = make_service(&pool);

        let res = service
            .get_key_by_id(&SshKeyId::try_new(9_999_999).unwrap())
            .await;
        assert!(res.is_err(), "should error for unknown id");
        assert!(res.err().unwrap().to_string().contains("SSH key not found"));
    }

    #[sqlx::test]
    async fn get_key_for_server_uses_env_for_local(pool: PgPool) {
        let service = make_service(&pool);

        let server = Server::Local {
            id: ServerId::try_new(1).unwrap(),
            name: "local".into(),
        };
        let key = service.get_key_for_server(&server).await.unwrap();

        match key {
            SshKey::KeyPair {
                username,
                private_key,
                ..
            } => {
                assert_eq!(username, "tester");
                assert!(private_key.as_str().starts_with("-----BEGIN"));
            }
            _ => panic!("expected keypair for local server"),
        }
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("ssh_keys")))]
    async fn get_key_for_server_fetches_for_remote(pool: PgPool) {
        let service = make_service(&pool);

        let id: i64 = sqlx::query_scalar!("SELECT id FROM ssh_keys WHERE name = $1", "kp-fixture")
            .fetch_one(&pool)
            .await
            .unwrap();

        // Construct a remote server referencing the seeded ssh key id
        let server = Server::Remote {
            id: ServerId::try_new(2).unwrap(),
            name: "remote".into(),
            hostname: "example.com".into(),
            ssh_port: "22".parse().unwrap(),
            ssh_key_id: SshKeyId::try_new(id).unwrap(),
        };

        let key = service.get_key_for_server(&server).await.unwrap();
        match key {
            SshKey::KeyPair { name, username, .. } => {
                assert_eq!(name, "kp-fixture");
                assert_eq!(username, "grace");
            }
            _ => panic!("expected keypair for remote server"),
        }
    }
}
