use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePool},
    ConnectOptions,
};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .disable_statement_logging();

    SqlitePool::connect_with(options).await
}
