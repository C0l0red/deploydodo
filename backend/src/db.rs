use sqlx::{
    postgres::{PgConnectOptions, PgPool},
    ConnectOptions,
};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let options = PgConnectOptions::from_str(database_url)?.disable_statement_logging();

    PgPool::connect_with(options).await
}
