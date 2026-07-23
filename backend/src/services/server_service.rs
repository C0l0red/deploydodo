use crate::impl_display_via_to_string;
use std::ops::Deref;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::new_types::ServerPort;
use crate::services::ssh_service::SshKeyId;
use crate::{
    entity, entity_id,
    env::get_env,
    error::{AppError, AppResult},
    impl_deref, impl_deserialize_via_try_new, newtype,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "servertype", rename_all = "lowercase")]
pub enum ServerType {
    Local,
    Remote,
}

impl ServerType {
    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local)
    }
}

#[derive(Debug)]
pub enum Server {
    Remote {
        id: ServerId,
        name: String,
        hostname: String,
        ssh_port: ServerPort,
        ssh_key_id: SshKeyId,
    },
    Local {
        id: ServerId,
        name: String,
    },
}

entity! {
    pub struct ServerRow {
        id: ServerId,
        name: String,
        server_type: ServerType,
        hostname: Option<String>,
        ssh_port: Option<ServerPort>,
        ssh_key_id: Option<SshKeyId>,
        created_at: DateTime<Utc>,
    }
}

impl ServerRowInput {
    pub(crate) fn local_server(name: String) -> Self {
        Self {
            name,
            hostname: None,
            ssh_port: None,
            ssh_key_id: None,
            server_type: ServerType::Local,
            created_at: Utc::now(),
        }
    }

    pub(crate) fn remote_server(
        name: String,
        hostname: String,
        ssh_port: ServerPort,
        ssh_key_id: SshKeyId,
    ) -> Self {
        Self {
            name,
            hostname: Some(hostname),
            ssh_port: Some(ssh_port),
            ssh_key_id: Some(ssh_key_id),
            server_type: ServerType::Remote,
            created_at: Utc::now(),
        }
    }
}

impl TryFrom<ServerRow> for Server {
    type Error = AppError;

    fn try_from(value: ServerRow) -> Result<Self, Self::Error> {
        Ok(match value.server_type {
            ServerType::Remote => Self::Remote {
                id: value.id,
                name: value.name,
                hostname: value
                    .hostname
                    .ok_or(AppError::CouldNotParse("hostname missing".to_string()))?,
                ssh_key_id: value
                    .ssh_key_id
                    .ok_or(AppError::CouldNotParse("ssh_key_id missing".to_string()))?,
                ssh_port: value
                    .ssh_port
                    .ok_or(AppError::CouldNotParse("ssh_port missing".to_string()))?,
            },
            ServerType::Local => Self::Local {
                id: value.id,
                name: value.name,
            },
        })
    }
}

impl<'a> Server {
    pub fn id(&self) -> ServerId {
        match self {
            Server::Local { id, .. } | Server::Remote { id, .. } => *id,
        }
    }

    pub fn name(&'a self) -> &'a str {
        match self {
            Server::Local { name, .. } | Server::Remote { name, .. } => name,
        }
    }

    pub fn server_type(&'a self) -> ServerType {
        match self {
            Server::Remote { .. } => ServerType::Remote,
            Server::Local { .. } => ServerType::Local,
        }
    }

    pub fn ssh_port(&self) -> ServerPort {
        match self {
            Server::Remote { ssh_port, .. } => *ssh_port,
            Server::Local { .. } => get_env().local_ssh_port,
        }
    }

    pub fn hostname(&self) -> String {
        match self {
            Server::Remote { hostname, .. } => hostname.to_owned(),
            Server::Local { .. } => get_env().local_ssh_hostname.to_owned(),
        }
    }
}

pub struct ServerService {
    db: Arc<PgPool>,
}

impl ServerService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn count_local_servers(&self) -> AppResult<i64> {
        Ok(
            sqlx::query_scalar!("SELECT COUNT(*) FROM servers WHERE server_type = 'local'")
                .fetch_one(&*self.db)
                .await?
                .unwrap_or(0),
        )
    }

    pub async fn create_server(&self, new_server_row: ServerRowInput) -> AppResult<Server> {
        let server_row = sqlx::query_as!(
            ServerRow,
            r#"
            INSERT INTO servers (name, hostname, ssh_port, ssh_key_id, server_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id AS "id!: _",
                name,
                hostname,
                ssh_port AS "ssh_port!: _",
                ssh_key_id AS "ssh_key_id!: _",
                server_type AS "server_type!: _",
                created_at AS "created_at!: _"
            "#,
            new_server_row.name,
            new_server_row.hostname,
            new_server_row.ssh_port.as_deref().copied().map(i32::from),
            new_server_row.ssh_key_id.as_deref(),
            new_server_row.server_type as ServerType,
            new_server_row.created_at,
        )
        .fetch_one(&*self.db)
        .await?;

        server_row.try_into()
    }

    pub async fn get_server_by_id(&self, server_id: ServerId) -> AppResult<Server> {
        let server_row = sqlx::query_as!(
            ServerRow,
            r#"
            SELECT
                id as "id!: ServerId",
                name,
                server_type AS "server_type!: ServerType",
                hostname,
                ssh_port AS "ssh_port?: ServerPort",
                ssh_key_id AS "ssh_key_id?: SshKeyId",
                created_at
            FROM servers WHERE id = $1
            "#,
            server_id.deref()
        )
        .fetch_optional(&*self.db)
        .await?
        .ok_or(AppError::not_found("Server not found"))?;

        server_row.try_into()
    }

    pub async fn list_servers(&self) -> AppResult<Vec<Server>> {
        let server_rows = sqlx::query_as!(
            ServerRow,
            r#"
            SELECT
                id as "id!: ServerId",
                name,
                server_type AS "server_type!: ServerType",
                hostname,
                ssh_port AS "ssh_port?: ServerPort",
                ssh_key_id AS "ssh_key_id?: SshKeyId",
                created_at
            FROM servers
            ORDER BY id
            "#,
        )
        .fetch_all(&*self.db)
        .await?;

        let servers = server_rows
            .into_iter()
            .map(TryInto::try_into)
            .collect::<AppResult<Vec<Server>>>()?;

        Ok(servers)
    }
}
