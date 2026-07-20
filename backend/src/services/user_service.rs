use std::ops::Deref;
use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use rand_core::OsRng;
use serde::Serialize;
use sqlx::{FromRow, PgPool, Type};
use utoipa::ToSchema;

use crate::new_types::{HashedPassword, PlainPassword};
use crate::{
    entity, entity_id,
    error::{AppError, AppResult},
    impl_deref, impl_deserialize_via_try_new, impl_display_via_to_string, newtype,
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

entity_id! {
    pub struct UserId;
}

entity! {
    #[derive(FromRow)]
    pub struct User {
        id: Option<i64>,
        name: String,
        email: String,
        password_hash: HashedPassword,
        account_type: AccountType,
        created_at: DateTime<Utc>,
    }
}

pub struct PasswordUtils;

impl PasswordUtils {
    pub fn hash_password(password: &PlainPassword) -> AppResult<HashedPassword> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordHash)
            .map(Into::into)
    }

    pub fn verify_password(
        plain_password: &PlainPassword,
        hashed_password: &HashedPassword,
    ) -> AppResult<()> {
        let parsed_hash = argon2::PasswordHash::new(hashed_password)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Argon2::default()
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)
    }
}

impl User {
    pub fn get_id(&self) -> AppResult<i64> {
        self.id.ok_or(AppError::InternalServerError(
            "id is None in User type".to_string(),
        ))
    }
}

impl NewUser {
    pub fn admin(
        create_admin_request: CreateAdminRequest,
        hashed_password: HashedPassword,
    ) -> Self {
        Self {
            name: create_admin_request.name.to_string(),
            email: create_admin_request.email.to_string(),
            password_hash: hashed_password,
            account_type: AccountType::Admin,
            created_at: Utc::now(),
        }
    }
}

impl UserService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, new_user: NewUser) -> AppResult<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password_hash, account_type, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                name,
                email,
                password_hash AS "password_hash: HashedPassword",
                account_type AS "account_type: AccountType",
                created_at
            "#,
            new_user.name,
            new_user.email,
            new_user.password_hash.deref(),
            new_user.account_type as AccountType,
            new_user.created_at
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
        Ok(sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                name, 
                email, 
                password_hash AS "password_hash: HashedPassword", 
                account_type AS "account_type: AccountType", 
                created_at 
            FROM users 
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*self.db)
        .await?)
    }
}
