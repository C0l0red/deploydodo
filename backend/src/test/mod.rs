use std::sync::Arc;

use crate::{dependencies::Dependencies, env, middleware};

use axum::{routing::MethodRouter, Router};
use axum_test::{TestRequest, TestServer};
use sqlx::{Pool, Postgres};
use std::sync::Once;

static INIT: Once = Once::new();

static TEST_ENDPOINT: &str = "/test-api";

pub struct App {
    pub deps: Dependencies,
    server: TestServer,
}

fn init_test_env() {
    std::env::set_var("LOCAL_SSH_PRIVATE_KEY", "../key_rsa");
    env::init_env();
}

impl App {
    pub fn get(&self) -> TestRequest {
        self.server.get(TEST_ENDPOINT)
    }

    pub fn post(&self) -> TestRequest {
        self.server.post(TEST_ENDPOINT)
    }

    pub fn delete(&self) -> TestRequest {
        self.server.delete(TEST_ENDPOINT)
    }

    pub fn patch(&self) -> TestRequest {
        self.server.patch(TEST_ENDPOINT)
    }

    pub async fn register_route(
        db: Pool<Postgres>,
        method_router: MethodRouter<Dependencies>,
    ) -> Self {
        INIT.call_once(init_test_env);
        let db = Arc::new(db);
        let deps =
            Dependencies::init_with_db(db.clone()).expect("failed to initialize dependencies");

        let router = Router::new()
            .route(TEST_ENDPOINT, method_router)
            .with_state(deps.clone())
            .layer(axum::middleware::from_fn(middleware::bearer_auth));

        let server = TestServer::new(router);

        App { deps, server }
    }
}
