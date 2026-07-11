#[tokio::main]
async fn main() {
    backend::env::init_env();

    let pool = backend::db::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
}
