use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

use crate::env::get_env;

pub async fn create_pool() -> sqlx::Result<PgPool> {
    let database_url = get_env().database_url.to_owned();

    let pool = connect_with_retry(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    Ok(pool)
}

async fn connect_with_retry(url: &str) -> sqlx::Result<PgPool> {
    let mut attempt = 0;
    loop {
        match PgPoolOptions::new().max_connections(5).connect(url).await {
            Ok(pool) => return Ok(pool),
            Err(e) if attempt < 10 => {
                attempt += 1;
                tracing::warn!("postgres not ready (attempt {attempt}): {e}");
                tokio::time::sleep(Duration::from_millis(500 * attempt)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
