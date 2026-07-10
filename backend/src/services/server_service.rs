use std::{sync::Arc, u16};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tokio::sync::OnceCell;
use utoipa::ToSchema;

use crate::error::{AppError, AppResult};

static LOCAL_SSH_PORT: OnceCell<u16> = OnceCell::const_new();
static LOCAL_SSH_HOSTNAME: OnceCell<String> = OnceCell::const_new();

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
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
        id: i64,
        name: String,
        hostname: String,
        ssh_port: u16,
        ssh_key_id: i64,
    },
    Local {
        id: i64,
        name: String,
    },
}

impl<'a> Server {
    pub fn id(&'a self) -> &'a i64 {
        match self {
            Server::Local { id, .. } | Server::Remote { id, .. } => id,
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

    pub fn ssh_port(&'a self) -> &'a u16 {
        match self {
            Server::Remote { ssh_port, .. } => ssh_port,
            Server::Local { .. } => LOCAL_SSH_PORT.get().unwrap(),
        }
    }

    pub fn hostname(&'a self) -> &'a str {
        match self {
            Server::Remote { hostname, .. } => hostname,
            Server::Local { .. } => LOCAL_SSH_HOSTNAME.get().unwrap(),
        }
    }
}

pub struct ServerService {
    db: Arc<SqlitePool>,
}

impl ServerService {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        let port = std::env::var("LOCAL_SSH_PORT")
            .map(|h| {
                h.parse::<u16>()
                    .expect("LOCAL_SSH_PORT must be a valid u16")
            })
            .expect("The variable LOCAL_SSH_PORT must be present at runtime");

        let hostname = std::env::var("LOCAL_SSH_HOSTNAME")
            .expect("The variable LOCAL_SSH_HOSTNAME must be present at runtime");

        LOCAL_SSH_PORT.set(port).unwrap();
        LOCAL_SSH_HOSTNAME.set(hostname).unwrap();

        Self { db }
    }

    pub async fn count_local_servers(&self) -> AppResult<i64> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM servers WHERE server_type = 'local'")
                .fetch_one(&*self.db)
                .await?,
        )
    }

    pub async fn create_local_server(&self, name: &str) -> AppResult<Server> {
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO servers (name, server_type, created_at) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(name)
        .bind(ServerType::Local)
        .bind(Utc::now())
        .fetch_one(&*self.db)
        .await?;

        Ok(Server::Local {
            id,
            name: name.to_string(),
        })
    }

    pub async fn get_server_by_id(&self, server_id: i64) -> AppResult<Server> {
        let row = sqlx::query(
            "SELECT id, name, server_type, hostname, ssh_port, ssh_key_id FROM servers WHERE id = $1",
        )
        .bind(server_id)
        .fetch_optional(&*self.db)
        .await
        ?
        .ok_or(AppError::Validation("Server not found".into()))?;

        let server_type: ServerType = row.try_get("server_type")?;

        Ok(if server_type.is_local() {
            Server::Local {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
            }
        } else {
            Server::Remote {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                hostname: row.try_get("hostname")?,
                ssh_port: row.try_get("ssh_port")?,
                ssh_key_id: row.try_get("ssh_key_id")?,
            }
        })
    }

    pub async fn list_servers(&self) -> AppResult<Vec<Server>> {
        let rows = sqlx::query(
            "SELECT id, name, server_type, hostname, ssh_port, ssh_key_id FROM servers ORDER BY id",
        )
        .fetch_all(&*self.db)
        .await?;

        let mut servers = vec![];
        for row in rows {
            let server_type: ServerType = row.try_get("server_type")?;

            let server = if server_type.is_local() {
                Server::Local {
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                }
            } else {
                Server::Remote {
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                    hostname: row.try_get("hostname")?,
                    ssh_port: row.try_get("ssh_port")?,
                    ssh_key_id: row.try_get("ssh_key_id")?,
                }
            };

            servers.push(server);
        }
        Ok(servers)
    }

    pub async fn create_remote_server(
        &self,
        name: &str,
        hostname: &str,
        ssh_port: u16,
        ssh_key_id: i64,
    ) -> AppResult<Server> {
        let server_id: i64 = sqlx::query_scalar(
            "INSERT INTO servers (name, server_type, hostname, ssh_port, ssh_key_id, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        )
        .bind(name)
        .bind("remote")
        .bind(hostname)
        .bind(ssh_port)
        .bind(ssh_key_id)
        .bind(Utc::now())
        .fetch_one(&*self.db)
        .await?;

        tracing::info!(id = %server_id, ssh_key_id = ssh_key_id, "remote server created");

        Ok(Server::Remote {
            id: server_id,
            name: name.to_string(),
            hostname: hostname.to_string(),
            ssh_port: ssh_port,
            ssh_key_id: ssh_key_id,
        })
    }
}
