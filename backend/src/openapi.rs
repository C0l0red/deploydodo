use utoipa::OpenApi;

use crate::routes::{
    create_admin, create_local_server, create_remote_server, health, job_events, list_servers,
    login, status, validate_session,
};
use crate::services::types;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "DeployDodo API",
        version = "0.1.0",
        description = "DeployDodo backend API"
    ),
    paths(
        health::health,
        status::status,
        validate_session::validate_session,
        login::login,
        create_admin::create_admin,
        create_local_server::create_local_server,
        create_remote_server::create_remote_server,
        job_events::job_events,
        list_servers::list_servers,
    ),
    components(schemas(
        health::HealthResponse,
        status::StatusResponse,
        validate_session::ValidateSessionResponse,
        login::LoginRequest,
        login::LoginResponse,
        create_admin::CreateAdminRequest,
        create_admin::AdminResponse,
        create_local_server::CreateLocalServerRequest,
        create_local_server::CreateLocalServerResponse,
        create_remote_server::CreateRemoteServerRequest,
        create_remote_server::SshAuthRequest,
        create_remote_server::StartJobResponse,
        list_servers::ServerResponse,
        types::AccountType,
        types::AuthType,
        types::ServerType,
        types::JobType,
    )),
    tags(
        (name = "health", description = "Health check"),
        (name = "status", description = "Config status"),
        (name = "auth", description = "Authentication"),
        (name = "setup", description = "Initial setup"),
        (name = "servers", description = "Server management"),
        (name = "jobs", description = "Background jobs"),
    )
)]
pub struct ApiDoc;
