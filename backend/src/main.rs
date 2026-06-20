mod db;
mod dependencies;
mod error;
mod openapi;
mod routes;
mod services;

use axum::{
    routing::{get, post},
    Json, Router,
};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;

use dependencies::Dependencies;

async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(openapi::ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
                tracing_subscriber::EnvFilter::new("backend=debug,tower_http=debug"),
            ),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let deps = Dependencies::init()
        .await
        .expect("failed to initialize dependencies");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(routes::health::health))
        .route("/api/status", get(routes::status::status))
        .route("/api/servers", get(routes::list_servers::list_servers))
        .route(
            "/api/auth/validate",
            get(routes::validate_session::validate_session),
        )
        .route(
            "/api/auth/login",
            post(routes::login::login),
        )
        .route("/api/setup/admin", post(routes::create_admin::create_admin))
        .route(
            "/api/setup/server/local",
            post(routes::create_local_server::create_local_server),
        )
        .route(
            "/api/setup/server/remote",
            post(routes::create_remote_server::create_remote_server),
        )
        .route(
            "/api/jobs/{job_id}/events",
            get(routes::job_events::job_events),
        )
        .route("/api/openapi.json", get(openapi_json))
        .with_state(deps)
        .layer(cors)
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
