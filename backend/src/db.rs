use sqlx::{
    postgres::{PgConnectOptions, PgPool},
    ConnectOptions,
};
use std::str::FromStr;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("The variable DATABASE_URL must be present at runtime");

    let options = PgConnectOptions::from_str(&database_url)?.disable_statement_logging();

    PgPool::connect_with(options).await
}
