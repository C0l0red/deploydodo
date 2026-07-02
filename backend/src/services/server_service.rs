use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use utoipa::ToSchema;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum ServerType {
    Local,
    Remote,
}

#[derive(Debug)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub server_type: ServerType,
    pub hostname: String,
    pub ssh_port: Option<u16>,
    pub ssh_key_id: Option<i64>,
}

pub struct ServerService {
    db: Arc<SqlitePool>,
}

impl ServerService {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn count_local_servers(&self) -> Result<i64, AppError> {
        sqlx::query_scalar("SELECT COUNT(*) FROM servers WHERE server_type = 'local'")
            .fetch_one(&*self.db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create_local_server(
        &self,
        name: &str,
        hostname: &str,
    ) -> Result<Server, AppError> {
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO servers (name, server_type, hostname, created_at) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(name)
        .bind(ServerType::Local)
        .bind(hostname)
        .bind(Utc::now())
        .fetch_one(&*self.db)
        .await
        .map_err(AppError::Database)?;

        Ok(Server {
            id,
            name: name.to_string(),
            server_type: ServerType::Local,
            hostname: hostname.to_string(),
            ssh_port: None,
            ssh_key_id: None,
        })
    }

    pub async fn get_server_by_id(&self, server_id: i64) -> Result<Server, AppError> {
        let row = sqlx::query(
            "SELECT id, name, server_type, hostname, ssh_port, ssh_key_id FROM servers WHERE id = $1",
        )
        .bind(server_id)
        .fetch_optional(&*self.db)
        .await
        .map_err(AppError::Database)?
        .ok_or(AppError::Validation("Server not found".into()))?;

        Ok(Server {
            id: row.try_get("id").map_err(AppError::Database)?,
            name: row.try_get("name").map_err(AppError::Database)?,
            server_type: row.try_get("server_type").map_err(AppError::Database)?,
            hostname: row.try_get("hostname").map_err(AppError::Database)?,
            ssh_port: row.try_get("ssh_port").map_err(AppError::Database)?,
            ssh_key_id: row.try_get("ssh_key_id").map_err(AppError::Database)?,
        })
    }

    pub async fn list_servers(&self) -> Result<Vec<Server>, AppError> {
        let rows = sqlx::query(
            "SELECT id, name, server_type, hostname, ssh_port, ssh_key_id FROM servers ORDER BY id",
        )
        .fetch_all(&*self.db)
        .await
        .map_err(AppError::Database)?;

        let mut servers = vec![];
        for row in rows {
            servers.push(Server {
                id: row.try_get("id").map_err(AppError::Database)?,
                name: row.try_get("name").map_err(AppError::Database)?,
                server_type: row.try_get("server_type").map_err(AppError::Database)?,
                hostname: row.try_get("hostname").map_err(AppError::Database)?,
                ssh_port: row.try_get("ssh_port").map_err(AppError::Database)?,
                ssh_key_id: row.try_get("ssh_key_id").map_err(AppError::Database)?,
            });
        }
        Ok(servers)
    }

    pub async fn create_remote_server(
        &self,
        name: &str,
        hostname: &str,
        ssh_port: u16,
        ssh_key_id: i64,
    ) -> Result<Server, AppError> {
        let server_id: i64 = sqlx::query_scalar(
            "INSERT INTO servers (name, server_type, hostname, ssh_port, ssh_key_id, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        )
        .bind(name)
        .bind(ServerType::Remote)
        .bind(hostname)
        .bind(ssh_port)
        .bind(ssh_key_id)
        .bind(Utc::now())
        .fetch_one(&*self.db)
        .await
        .map_err(AppError::Database)?;

        Ok(Server {
            id: server_id,
            name: name.to_string(),
            server_type: ServerType::Remote,
            hostname: hostname.to_string(),
            ssh_port: Some(ssh_port),
            ssh_key_id: Some(ssh_key_id),
        })
    }
}
