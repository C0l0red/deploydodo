use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};

use crate::dependencies::Dependencies;
use crate::services::terminal_service::TerminalService;
use crate::services::types::ServerType;

// ── JSON protocol ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum ClientMessage {
    Run { container: String, cmd: String },
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum ServerMessage {
    Stdout { data: String },
    Stderr { data: String },
    Error { message: String },
    Done,
    Cd { dir: String },
}

#[derive(Deserialize)]
pub struct TerminalParams {
    pub token: String,
}

// ── Public handler ────────────────────────────────────────────────────────────

pub async fn terminal_ws(
    ws: axum::extract::ws::WebSocketUpgrade,
    axum::extract::Query(params): axum::extract::Query<TerminalParams>,
    axum::extract::State(deps): axum::extract::State<Dependencies>,
    axum::extract::Path(server_id): axum::extract::Path<i64>,
) -> impl axum::response::IntoResponse {
    let token = params.token.clone();
    ws.on_upgrade(move |socket| async move {
        if let Err(e) = handle_socket(socket, deps, server_id, token).await {
            tracing::error!(error = %e, "terminal ws error");
        }
    })
}

async fn handle_socket(
    mut socket: WebSocket,
    deps: Dependencies,
    server_id: i64,
    token: String,
) -> Result<(), crate::error::AppError> {
    // Auth
    let valid = deps.session_service.validate_session(token.trim()).await?;
    if !valid {
        let _ = send_msg(&mut socket, ServerMessage::Error { message: "Unauthorized".into() }).await;
        let _ = socket.close().await;
        return Ok(());
    }

    // Look up server
    let servers = deps.server_service.list_servers().await?;
    let server = servers.into_iter().find(|s| s.id == server_id).ok_or(
        crate::error::AppError::Validation("Server not found".into()),
    )?;

    let ssh_key = if server.server_type == ServerType::Remote {
        let key_id =
            server
                .ssh_key_id
                .ok_or(crate::error::AppError::Validation(
                    "No SSH key for remote server".into(),
                ))?;
        Some(deps.ssh_service.get_key_by_id(key_id).await?)
    } else {
        None
    };
    let session = TerminalService::connect(&server, ssh_key.as_ref()).await?;
    let docker = &session.docker;

    let mut current_dir = String::from("/");

    // Process commands from client
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                let client_msg = serde_json::from_str::<ClientMessage>(&text);
                match client_msg {
                    Ok(ClientMessage::Run { container, cmd }) => {
                        let current_container = container.clone();

                        // Handle cd locally (state tracking)
                        let cmd_trimmed = cmd.trim();
                        if cmd_trimmed == "cd" || cmd_trimmed == "cd ~" {
                            current_dir = String::from("/root");
                            let _ = send_msg(
                                &mut socket,
                                ServerMessage::Cd {
                                    dir: current_dir.clone(),
                                },
                            )
                            .await;
                            continue;
                        }
                        if cmd_trimmed == "cd /" {
                            current_dir = String::from("/");
                            let _ = send_msg(
                                &mut socket,
                                ServerMessage::Cd {
                                    dir: current_dir.clone(),
                                },
                            )
                            .await;
                            continue;
                        }
                        if cmd_trimmed == "cd .." {
                            if let Some(parent) =
                                std::path::Path::new(&current_dir).parent()
                            {
                                current_dir =
                                    parent.to_string_lossy().to_string();
                            }
                            if current_dir.is_empty() {
                                current_dir = String::from("/");
                            }
                            let _ = send_msg(
                                &mut socket,
                                ServerMessage::Cd {
                                    dir: current_dir.clone(),
                                },
                            )
                            .await;
                            continue;
                        }
                        if let Some(target) =
                            cmd_trimmed.strip_prefix("cd ")
                        {
                            let target = target.trim().trim_matches('"').trim_matches('\'');
                            if target.starts_with('/') {
                                current_dir = target.to_string();
                            } else if target == ".." {
                                if let Some(parent) =
                                    std::path::Path::new(&current_dir).parent()
                                {
                                    current_dir =
                                        parent.to_string_lossy().to_string();
                                }
                                if current_dir.is_empty() {
                                    current_dir = String::from("/");
                                }
                            } else {
                                current_dir = format!(
                                    "{}/{}",
                                    current_dir.trim_end_matches('/'),
                                    target
                                );
                            }
                            let _ = send_msg(
                                &mut socket,
                                ServerMessage::Cd {
                                    dir: current_dir.clone(),
                                },
                            )
                            .await;
                            continue;
                        }

                        // Build the command: cd to current dir, set color env, wrap ls
                        let full_cmd = build_command(&current_dir, &cmd);

                        match TerminalService::run_command(
                            docker,
                            &current_container,
                            &full_cmd,
                        )
                        .await
                        {
                            Ok(outputs) => {
                                for out in outputs {
                                    match out {
                                        crate::services::terminal_service::TerminalOutput::Stdout(data) => {
                                            let _ = send_msg(&mut socket, ServerMessage::Stdout { data }).await;
                                        }
                                        crate::services::terminal_service::TerminalOutput::Stderr(data) => {
                                            let _ = send_msg(&mut socket, ServerMessage::Stderr { data }).await;
                                        }
                                    }
                                }
                                let _ = send_msg(&mut socket, ServerMessage::Done).await;
                            }
                            Err(e) => {
                                let _ = send_msg(
                                    &mut socket,
                                    ServerMessage::Error {
                                        message: e.to_string(),
                                    },
                                ).await;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "invalid client message");
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    let _ = socket.close().await;
    Ok(())
}

async fn send_msg(
    socket: &mut WebSocket,
    msg: ServerMessage,
) -> Result<(), crate::error::AppError> {
    let json = serde_json::to_string(&msg).unwrap_or_default();
    socket
        .send(Message::Text(json.into()))
        .await
        .map_err(|e| crate::error::AppError::DockerConnection(e.to_string()))
}

/// Builds the full shell command with directory context, color env vars,
/// and ls formatting fixes so output looks like a real terminal.
fn build_command(current_dir: &str, user_cmd: &str) -> String {
    if user_cmd.trim().is_empty() {
        return format!("cd {} && true", shell_escape(current_dir));
    }

    format!(
        "{setup} && {user_cmd}",
        setup = format!(
            "cd {} && export TERM=xterm-256color CLICOLOR_FORCE=1 && ls() {{ command ls -C --color=always \"$@\"; }}",
            shell_escape(current_dir),
        ),
        user_cmd = user_cmd,
    )
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
