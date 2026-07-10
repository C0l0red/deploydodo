use std::sync::Arc;

use chrono::Utc;
use rand_core::{OsRng, RngCore};
use sqlx::SqlitePool;

use crate::error::AppResult;

pub struct SessionService {
    db: Arc<SqlitePool>,
}

impl SessionService {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn validate_session(&self, token: &str) -> AppResult<bool> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT * FROM auth_sessions WHERE session_token = $1)",
        )
        .bind(token)
        .fetch_one(&*self.db)
        .await?;

        Ok(exists)
    }

    pub async fn create_session(&self, user_id: i64) -> AppResult<String> {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let token = bytes.map(|b| format!("{b:02x}")).concat();

        sqlx::query(
            "INSERT INTO auth_sessions (user_id, session_token, created_at) VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(&token)
        .bind(Utc::now())
        .execute(&*self.db)
        .await?;

        Ok(token)
    }
}
