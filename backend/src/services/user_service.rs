use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use rand_core::OsRng;
use serde::Serialize;
use sqlx::{FromRow, PgPool, Type};
use utoipa::ToSchema;

use crate::{
    error::{AppError, AppResult},
    routes::create_admin::CreateAdminRequest,
};

pub struct UserService {
    db: Arc<PgPool>,
}

#[derive(Type, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "accounttype", rename_all = "lowercase")]
pub enum AccountType {
    Admin,
    Member,
}

#[derive(FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub account_type: AccountType,
    pub created_at: DateTime<chrono::Utc>,
}

impl User {
    pub fn verify_password(&self, password: &str) -> AppResult<()> {
        let parsed_hash = argon2::PasswordHash::new(&self.password_hash)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)
    }

    pub fn get_id(&self) -> AppResult<i64> {
        self.id.ok_or(AppError::InternalServerError(
            "id is None in User type".to_string(),
        ))
    }
}

impl From<CreateAdminRequest> for User {
    fn from(value: CreateAdminRequest) -> Self {
        User {
            id: None,
            name: value.name,
            email: value.email,
            password_hash: value.password,
            account_type: AccountType::Admin,
            created_at: Utc::now(),
        }
    }
}

impl UserService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, mut user: User) -> AppResult<User> {
        user.password_hash = Self::hash_password(&user.password_hash)?;

        Ok(sqlx::query_as!(User,
            r#"INSERT INTO users (name, email, password_hash, account_type, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, email, password_hash, account_type AS "account_type: AccountType", created_at"#,
            &user.name,
            &user.email,
            &user.password_hash,
            &user.account_type as _,
            &user.created_at
        )
        .fetch_one(&*self.db)
        .await?)
    }

    pub async fn count_users(&self) -> AppResult<i64> {
        Ok(sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(&*self.db)
            .await?
            .unwrap_or(0))
    }

    pub async fn get_by_email(&self, email: &str) -> AppResult<Option<User>> {
        Ok(sqlx::query_as!(User,
            r#"SELECT id, name, email, password_hash, account_type AS "account_type: AccountType", created_at FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&*self.db)
        .await?)
    }

    fn hash_password(password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordHash)
            .map(|hash| hash.to_string())
    }
}
