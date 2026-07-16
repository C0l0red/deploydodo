use axum::{extract::State, http::StatusCode, Json};
use dodosh::{SshAuth, SshSession, SshTimeout};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::dependencies::Dependencies;
use crate::error::{AppError, AppResult};
use crate::extractors::Auth;
use crate::services::ssh_service::SshKey;
use crate::services::types::{JobStatus, JobType};

// ── SSH auth sub-types ────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
#[serde(tag = "authType", rename_all = "lowercase")]
pub enum SshAuthRequest {
    Password {
        username: String,
        password: String,
    },
    KeyPair {
        username: String,
        #[serde(rename = "privateKey")]
        private_key: String,
        #[serde(rename = "publicKey")]
        public_key: Option<String>,
    },
}

impl SshAuthRequest {
    pub fn validate(&self) -> AppResult<()> {
        match self {
            SshAuthRequest::Password { username, password } => {
                if username.trim().is_empty() {
                    return Err(AppError::Validation("Username is required".into()));
                }
                if password.is_empty() {
                    return Err(AppError::Validation("Password is required".into()));
                }
            }
            SshAuthRequest::KeyPair {
                username,
                private_key,
                ..
            } => {
                if username.trim().is_empty() {
                    return Err(AppError::Validation("Username is required".into()));
                }
                if private_key.trim().is_empty() {
                    return Err(AppError::Validation("Private key is required".into()));
                }
            }
        }
        Ok(())
    }

    pub fn get_username(&self) -> &str {
        match self {
            Self::Password { username, .. } | Self::KeyPair { username, .. } => username,
        }
    }

    pub fn get_ssh_auth<'a>(&'a self) -> SshAuth<'a> {
        match self {
            Self::Password { password, .. } => SshAuth::Password(password),
            Self::KeyPair { private_key, .. } => SshAuth::Key {
                private_key,
                passphrase: None,
            },
        }
    }
}

// ── Request / Response ────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateRemoteServerRequest {
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub auth: SshAuthRequest,
}

impl CreateRemoteServerRequest {
    fn validate(&self) -> AppResult<()> {
        if self.name.trim().is_empty() {
            return Err(AppError::Validation("Name is required".into()));
        }
        if self.hostname.trim().is_empty() {
            return Err(AppError::Validation("Hostname is required".into()));
        }
        self.auth.validate()
    }
}

/// Returned immediately — use `jobId` to stream progress via
/// `GET /api/jobs/:jobId/events`.
#[derive(Serialize, ToSchema)]
pub struct StartJobResponse {
    #[serde(rename = "jobId")]
    pub job_id: String,
}

// ── Checklist step types (used by the frontend progress UI) ──────────────────

#[derive(Debug, Serialize, Deserialize)]
struct ConnectingStep {
    key: StepKey,
    label: String,
    status: CheckListItemStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckListItemStatus {
    Warning,
    Pending,
    Loading,
    Done,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum StepKey {
    InitiatingSsh,
    CheckingRoot,
    InstallingDocker,
}

fn create_connecting_remote_steps(active_key: StepKey) -> Vec<ConnectingStep> {
    let mut steps = vec![
        ConnectingStep {
            key: StepKey::InitiatingSsh,
            label: "Initiating SSH connection".into(),
            status: CheckListItemStatus::Pending,
        },
        ConnectingStep {
            key: StepKey::CheckingRoot,
            label: "Checking root permissions".into(),
            status: CheckListItemStatus::Pending,
        },
        ConnectingStep {
            key: StepKey::InstallingDocker,
            label: "Verifying Docker installation".into(),
            status: CheckListItemStatus::Pending,
        },
    ];

    let active_index = steps.iter().position(|s| s.key == active_key);

    if let Some(active_index) = active_index {
        for (i, step) in steps.iter_mut().enumerate() {
            step.status = if i < active_index {
                CheckListItemStatus::Done
            } else if i == active_index {
                CheckListItemStatus::Loading
            } else {
                CheckListItemStatus::Pending
            };
        }
    }

    steps
}

#[utoipa::path(
    post,
    path = "/api/setup/server/remote",
    params(
        ("Authorization" = String, Header, description = "authorization token")
    ),
    request_body = CreateRemoteServerRequest,
    responses(
        (status = 202, description = "Job accepted — stream progress via /api/jobs/{jobId}/events", body = StartJobResponse),
    ),
    tag = "setup"
)]
pub async fn create_remote_server(
    _: Auth,
    State(deps): State<Dependencies>,
    Json(request): Json<CreateRemoteServerRequest>,
) -> AppResult<(StatusCode, Json<StartJobResponse>)> {
    request.validate()?;

    let job_id = deps.job_service.create_job(JobType::CreateServer).await?;

    let job_id_bg = job_id.clone();
    tokio::spawn(async move {
        run_job(job_id_bg, request, deps).await;
    });

    Ok((StatusCode::ACCEPTED, Json(StartJobResponse { job_id })))
}

async fn run_job(job_id: String, request: CreateRemoteServerRequest, deps: Dependencies) {
    match handle_remote(&job_id, &request, &deps).await {
        Ok(()) => {
            let _ = deps
                .job_service
                .finish_job(&job_id, JobStatus::Completed)
                .await;
        }
        Err(e) => {
            tracing::error!("{e}");
            let _ = deps
                .job_service
                .emit(&job_id, "error", json!({ "message": e.to_string() }))
                .await;
            let _ = deps
                .job_service
                .finish_job(&job_id, JobStatus::Failed)
                .await;
        }
    }
}

async fn handle_remote(
    job_id: &str,
    request: &CreateRemoteServerRequest,
    deps: &Dependencies,
) -> AppResult<()> {
    let CreateRemoteServerRequest {
        name,
        hostname,
        port,
        auth,
    } = request;

    deps.job_service
        .emit(
            job_id,
            "progress",
            json!({
                "steps": create_connecting_remote_steps(StepKey::InitiatingSsh),
            }),
        )
        .await?;

    let session = SshSession::connect(
        hostname,
        *port,
        auth.get_username(),
        auth.get_ssh_auth(),
        SshTimeout::inactivity_secs(300),
    )
    .await?;

    deps.job_service
        .emit(
            job_id,
            "progress",
            json!({
                "steps": create_connecting_remote_steps(StepKey::CheckingRoot),
            }),
        )
        .await?;

    let is_root = session.check_root_access().await.map_err(AppError::Ssh)?;
    if !is_root {
        session.disconnect().await?;
        return Err(AppError::Validation("Root access required".to_string()));
    }

    deps.job_service
        .emit(
            job_id,
            "progress",
            json!({
                "steps": create_connecting_remote_steps(StepKey::InstallingDocker),
            }),
        )
        .await?;

    verify_docker_runtime(&session, true).await?;

    session.disconnect().await?;

    let key_name = format!("{name}-key");
    let ssh_key = create_ssh_key(&key_name, auth, deps).await?;

    let server = deps
        .server_service
        .create_remote_server(name, hostname, *port, *ssh_key.id())
        .await?;

    tracing::info!(id = %&server.id(), ssh_key_id = ssh_key.id(), "remote server created");

    deps.job_service
        .emit(
            job_id,
            "complete",
            json!({
                "id": server.id(),
                "name": server.name(),
                "serverType": server.server_type(),
                "hostname": server.hostname(),
                "port": server.ssh_port(),
            }),
        )
        .await?;

    Ok(())
}

async fn create_ssh_key(
    key_name: &str,
    auth: &SshAuthRequest,
    deps: &Dependencies,
) -> AppResult<SshKey> {
    match auth {
        SshAuthRequest::Password { username, password } => {
            deps.ssh_service
                .create_password_auth(key_name, username, password)
                .await
        }
        SshAuthRequest::KeyPair {
            username,
            private_key,
            public_key,
        } => {
            deps.ssh_service
                .create_key_auth(key_name, username, private_key, public_key.as_deref())
                .await
        }
    }
}

async fn verify_docker_runtime(session: &SshSession, retry_after_install: bool) -> AppResult<()> {
    let docker_status = session.check_docker().await.map_err(AppError::Ssh)?;
    if docker_status.is_installed {
        let is_docker_installed_via_snap = session
            .is_docker_installed_via_snap()
            .await
            .map_err(AppError::Ssh)?;
        if is_docker_installed_via_snap {
            session.disconnect().await?;
            return Err(AppError::Validation(
                "Docker runtime is present but it was installed via snap.
                Please remove the snap installation and use the system package manager instead."
                    .to_string(),
            ));
        }
        if !docker_status.is_running {
            session.disconnect().await?;
            return Err(AppError::Validation(
                "Docker runtime is installed but not running".to_string(),
            ));
        }
        Ok(())
    } else {
        install_docker(session).await?;
        if retry_after_install {
            Box::pin(verify_docker_runtime(session, false)).await?;
        }
        Ok(())
    }
}

async fn install_docker(session: &SshSession) -> AppResult<()> {
    let output = session
        .run_command("curl -fsSL https://get.docker.com -o get-docker.sh")
        .await?;

    if output.exit_code != 0 {
        return Err(AppError::Validation(format!(
            "Failed to download Docker installation script: {}",
            output.stdout
        )));
    }

    let output = session.run_command("sh get-docker.sh").await?;

    let _ = session.run_command("rm get-docker.sh").await;

    if output.exit_code != 0 {
        return Err(AppError::Validation(format!(
            "Failed to install Docker: {}",
            output.stdout
        )));
    }

    Ok(())
}
