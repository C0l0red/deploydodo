#[tokio::main]
async fn main() {
    let pool = backend::db::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
}
