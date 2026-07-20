use backend::dependencies;
use backend::env;
use backend::middleware;
use backend::routes;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use dependencies::Dependencies;

fn build_routes(deps: Dependencies) -> Router {
    let index_resource = ServeFile::new("dist/index.html");

    let static_files = ServeDir::new("dist").not_found_service(index_resource);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .fallback_service(static_files)
        .route("/api/health", get(routes::health::health))
        .route("/api/status", get(routes::status::status))
        .route("/api/servers", get(routes::list_servers::list_servers))
        .route(
            "/api/servers/{server_id}/terminal",
            get(routes::terminal::terminal_ws),
        )
        .route(
            "/api/auth/validate",
            get(routes::validate_session::validate_session),
        )
        .route("/api/auth/login", post(routes::login::login))
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
        .with_state(deps)
        .layer(axum::middleware::from_fn(middleware::bearer_auth))
        .layer(cors)
        .layer(CompressionLayer::new())
}

#[tokio::main]
async fn main() {
    env::init_env();

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

    let app = build_routes(deps);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
