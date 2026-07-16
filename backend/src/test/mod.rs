use std::sync::Arc;

use crate::{dependencies::Dependencies, env, middleware};

use axum::{routing::MethodRouter, Router};
use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use std::sync::Once;

static INIT: Once = Once::new();

pub struct App {
    pub db: Arc<Pool<Postgres>>,
    pub server: TestServer,
}

fn init_test_env() {
    std::env::set_var("LOCAL_SSH_PRIVATE_KEY", "../key_rsa");
    env::init_env();
}

impl App {
    pub async fn register_route(
        db: Pool<Postgres>,
        path: &str,
        method_router: MethodRouter<Dependencies>,
    ) -> Self {
        INIT.call_once(init_test_env);
        let db = Arc::new(db);
        let deps =
            Dependencies::init_with_db(db.clone()).expect("failed to initialize dependencies");

        let router = Router::new()
            .route(path, method_router)
            .with_state(deps)
            .layer(axum::middleware::from_fn(middleware::bearer_auth));

        let server = TestServer::new(router);

        App { db: db, server }
    }
}
