use crate::new_types::HashedPassword;
use crate::services::types::AccountType;
use std::ops::Deref;
use std::sync::Arc;

use chrono::Utc;
use rand_core::{OsRng, RngCore};
use sqlx::PgPool;

use crate::error::AppResult;
use crate::middleware::BearerToken;
use crate::services::types::User;
use crate::services::user_service::UserId;

pub struct SessionService {
    db: Arc<PgPool>,
}

impl SessionService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn create_session(&self, user_id: UserId) -> AppResult<String> {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let token = bytes.map(|b| format!("{b:02x}")).concat();

        sqlx::query!(
            "INSERT INTO auth_sessions (user_id, session_token, created_at) VALUES ($1, $2, $3)",
            user_id.deref(),
            &token,
            Utc::now()
        )
        .execute(&*self.db)
        .await?;

        Ok(token)
    }

    // FIXME: Should this be here on in the user service? As it returns a user
    pub async fn get_session_user(&self, token: &BearerToken) -> AppResult<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            r#"
                SELECT
                    id AS "id: UserId",
                    name,
                    email,
                    created_at,
                    password_hash AS "password_hash: HashedPassword",
                    account_type AS "account_type: AccountType"
                FROM users
                WHERE id = (
                    SELECT user_id
                    FROM auth_sessions
                    WHERE session_token = $1
                    LIMIT 1
                    )
                "#,
            token.deref()
        )
        .fetch_optional(&*self.db)
        .await?)
    }
}
