use std::sync::Arc;

use chrono::Utc;
use sqlx::PgPool;

use crate::error::AppResult;

pub struct VariablesService {
    db: Arc<PgPool>,
}

impl VariablesService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn get(&self, name: &str) -> AppResult<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT value FROM variables WHERE name = $1")
                .bind(name)
                .fetch_optional(&*self.db)
                .await?,
        )
    }

    pub async fn set(&self, name: &str, value: &str) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO variables (name, value, created_at) VALUES ($1, $2, $3)
             ON CONFLICT(name) DO UPDATE SET value = excluded.value",
        )
        .bind(name)
        .bind(value)
        .bind(Utc::now())
        .execute(&*self.db)
        .await?;

        Ok(())
    }
}
