use std::{sync::Arc, u16};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{
    env::get_env,
    error::{AppError, AppResult},
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

    pub fn ssh_port(&self) -> u16 {
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

    pub async fn create_local_server(&self, name: &str) -> AppResult<Server> {
        let id: i64 = sqlx::query_scalar!(
            "INSERT INTO servers (name, server_type, created_at) VALUES ($1, $2, $3) RETURNING id",
            name,
            ServerType::Local as _,
            Utc::now()
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(Server::Local {
            id,
            name: name.to_string(),
        })
    }

    pub async fn get_server_by_id(&self, server_id: i64) -> AppResult<Server> {
        let row = sqlx::query!(
            r#"SELECT id, name, server_type AS "server_type!: ServerType", hostname, ssh_port, ssh_key_id FROM servers WHERE id = $1"#,
        server_id)
        .fetch_optional(&*self.db)
        .await
        ?
        .ok_or(AppError::not_found("Server not found"))?;

        Ok(if row.server_type.is_local() {
            Server::Local {
                id: row.id,
                name: row.name,
            }
        } else {
            let port: i32 = row
                .ssh_port
                .ok_or(AppError::validation("ssh_port is None for server type"))?;
            Server::Remote {
                id: row.id,
                name: row.name,
                hostname: row
                    .hostname
                    .ok_or(AppError::validation("hostname is None for server type"))?,
                ssh_port: port as u16,
                ssh_key_id: row
                    .ssh_key_id
                    .ok_or(AppError::validation("ssh_key_id is None for server type"))?,
            }
        })
    }

    pub async fn list_servers(&self) -> AppResult<Vec<Server>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, server_type AS "server_type!: ServerType", hostname, ssh_port, ssh_key_id FROM servers ORDER BY id"#,
        )
        .fetch_all(&*self.db)
        .await?;

        let mut servers = vec![];
        for row in rows {
            let server = if row.server_type.is_local() {
                Server::Local {
                    id: row.id,
                    name: row.name,
                }
            } else {
                let port: i32 = row
                    .ssh_port
                    .ok_or(AppError::validation("ssh_port is None for server type"))?;
                Server::Remote {
                    id: row.id,
                    name: row.name,
                    hostname: row
                        .hostname
                        .ok_or(AppError::validation("hostname is None for server type"))?,
                    ssh_port: port as u16,
                    ssh_key_id: row
                        .ssh_key_id
                        .ok_or(AppError::validation("ssh_key_id is None for server type"))?,
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
        let server_id: i64 = sqlx::query_scalar!(
            "INSERT INTO servers (name, server_type, hostname, ssh_port, ssh_key_id, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            name,
            ServerType::Remote as _,
            hostname,
            ssh_port as i32,
            ssh_key_id,
            Utc::now()
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(Server::Remote {
            id: server_id,
            name: name.to_string(),
            hostname: hostname.to_string(),
            ssh_port,
            ssh_key_id,
        })
    }
}
