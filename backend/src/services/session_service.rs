use std::sync::Arc;

use chrono::Utc;
use rand_core::{OsRng, RngCore};
use sqlx::PgPool;

use crate::{
    error::AppResult,
    services::types::{AccountType, User},
};

pub struct SessionService {
    db: Arc<PgPool>,
}

impl SessionService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn get_session_user(&self, token: &str) -> AppResult<Option<User>> {
        Ok(
            sqlx::query_as!(User, r#"SELECT id, name, email, created_at, password_hash, account_type AS "account_type: AccountType" FROM users WHERE id = (SELECT user_id FROM auth_sessions WHERE session_token = $1 LIMIT 1)"#, token)
                .fetch_optional(&*self.db)
                .await?,
        )
    }

    pub async fn create_session(&self, user_id: i64) -> AppResult<String> {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let token = bytes.map(|b| format!("{b:02x}")).concat();

        sqlx::query!(
            "INSERT INTO auth_sessions (user_id, session_token, created_at) VALUES ($1, $2, $3)",
            user_id,
            &token,
            Utc::now()
        )
        .execute(&*self.db)
        .await?;

        Ok(token)
    }
}
