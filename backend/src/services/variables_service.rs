use std::sync::Arc;

use chrono::Utc;
use sqlx::{prelude::FromRow, PgPool};

use crate::error::AppResult;

pub trait VariableValueByKey {
    fn get_boolean(&self, key: VariableKey) -> Option<bool>;
}

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
pub enum VariableKey {
    IsAdminOnboarded,
    IsServerSetup,
    IsLocalServerSetup,
    IsProjectSetup,
}

#[derive(Debug, FromRow)]
pub struct Variable {
    pub name: VariableKey,
    pub value: String,
}

impl VariableValueByKey for Vec<Variable> {
    fn get_boolean(&self, key: VariableKey) -> Option<bool> {
        self.iter().find_map(move |var| {
            if key == var.name {
                var.value.parse::<bool>().ok()
            } else {
                None
            }
        })
    }
}

pub struct VariablesService {
    db: Arc<PgPool>,
}

impl VariablesService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self, keys: Vec<VariableKey>) -> AppResult<Vec<Variable>> {
        Ok(
            sqlx::query_as("SELECT name, value FROM variables WHERE name = ANY($1)")
                .bind(keys)
                .fetch_all(&*self.db)
                .await?,
        )
    }

    async fn set(&self, name: VariableKey, value: String) -> AppResult<()> {
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

    pub async fn set_value<V: ToString>(&self, name: VariableKey, value: V) -> AppResult<()> {
        self.set(name, value.to_string()).await
    }
}
