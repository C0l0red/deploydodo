use std::ops::Deref;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool, Type};
use utoipa::ToSchema;

use crate::new_types::HashedPassword;
use crate::{
    entity, entity_id,
    error::{AppError, AppResult},
    impl_deref, impl_deserialize_via_try_new, impl_display_via_to_string, newtype,
    routes::create_admin::CreateAdminRequest,
};

pub struct UserService {
    db: Arc<PgPool>,
}

#[derive(Type, Serialize, ToSchema, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "accounttype", rename_all = "lowercase")]
pub enum AccountType {
    Admin,
    Member,
}

entity! {
    #[derive(FromRow)]
    pub struct User {
        id: UserId,
        name: String,
        email: String,
        password_hash: HashedPassword,
        account_type: AccountType,
        created_at: DateTime<Utc>,
    }
}

impl UserInput {
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

    pub async fn create_user(&self, user_input: UserInput) -> AppResult<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password_hash, account_type, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id AS "id: UserId",
                name,
                email,
                password_hash AS "password_hash: HashedPassword",
                account_type AS "account_type: AccountType",
                created_at
            "#,
            user_input.name,
            user_input.email,
            user_input.password_hash.deref(),
            user_input.account_type as AccountType,
            user_input.created_at
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
                id AS "id: UserId",
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

#[cfg(test)]
mod test {
    use crate::new_types::HashedPassword;
    use crate::services::user_service::UserId;
    use crate::services::user_service::{AccountType, User, UserInput};
    use crate::services::UserService;
    use std::sync::Arc;

    fn make_user_service(pool: &sqlx::PgPool) -> UserService {
        UserService::new(Arc::new(pool.clone()))
    }

    #[sqlx::test]
    async fn create_user_persists_a_user(pool: sqlx::PgPool) {
        let user_service = make_user_service(&pool);
        let password_hash =
            HashedPassword::hash(&"test_password".into()).expect("Failed to hash password");
        let user_input = UserInput {
            name: "test".to_string(),
            email: "test@test.com".to_string(),
            password_hash,
            account_type: AccountType::Admin,
            created_at: Default::default(),
        };

        user_service.create_user(user_input).await.unwrap();

        let db_user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id AS "id: UserId",
                name,
                email,
                password_hash AS "password_hash: HashedPassword",
                account_type AS "account_type: AccountType",
                created_at
            FROM users
            LIMIT 1
            "#
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(db_user.name, "test");
        assert_eq!(db_user.email, "test@test.com");
        assert_eq!(db_user.account_type, AccountType::Admin);
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("users")))]
    async fn count_users_counts_rows_from_fixtures(pool: sqlx::PgPool) {
        let user_service = make_user_service(&pool);

        let count = user_service.count_users().await.unwrap();

        assert_eq!(count, 2, "there should be exactly 2 users from fixtures");
    }

    // Verify an existing user can be fetched by email
    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("users")))]
    async fn get_by_email_returns_user_when_present(pool: sqlx::PgPool) {
        let user_service = make_user_service(&pool);

        let found = user_service.get_by_email("ada@example.com").await.unwrap();

        let user = found.expect("expected user for ada@example.com");
        assert_eq!(user.email, "ada@example.com");
        assert_eq!(user.name, "Ada Lovelace");
        assert_eq!(user.account_type, AccountType::Admin);
    }

    // Unknown email should return None
    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("users")))]
    async fn get_by_email_returns_none_for_unknown(pool: sqlx::PgPool) {
        let user_service = make_user_service(&pool);

        let found = user_service
            .get_by_email("nobody@example.com")
            .await
            .unwrap();

        assert!(found.is_none(), "expected None for unknown email");
    }
}
