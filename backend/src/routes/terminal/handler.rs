use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;

use super::command::build_command;
use super::directory::resolve_cd;
use super::messaging::send_msg;
use super::session::establish_terminal_session;
use super::types::{ClientMessage, ServerMessage};
use crate::dependencies::Dependencies;
use crate::services::terminal_service::{TerminalOutput, TerminalSession};

pub async fn handle_socket(
    mut socket: WebSocket,
    server_id: i64,
    token: String,
    deps: Dependencies,
) -> Result<(), crate::error::AppError> {
    let session = establish_terminal_session(server_id, &token, &deps).await?;
    let mut current_dir = String::from("/");

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                process_text_message(
                    &mut socket,
                    &text,
                    &mut current_dir,
                    &session,
                )
                .await;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    let _ = socket.close().await;
    Ok(())
}

async fn process_text_message(
    socket: &mut WebSocket,
    text: &str,
    current_dir: &mut String,
    session: &TerminalSession,
) {
    let client_msg = match serde_json::from_str::<ClientMessage>(text) {
        Ok(msg) => msg,
        Err(e) => {
            tracing::warn!(error = %e, "invalid client message");
            return;
        }
    };

    match client_msg {
        ClientMessage::Run { container, cmd } => {
            let cmd_trimmed = cmd.trim();

            if let Some(new_dir) = try_handle_cd(socket, current_dir, cmd_trimmed).await {
                *current_dir = new_dir;
                return;
            }

            let full_cmd = build_command(current_dir, cmd_trimmed);
            execute_in_container(socket, session, &container, &full_cmd).await;
        }
        ClientMessage::Ping => {}
    }
}

async fn try_handle_cd(
    socket: &mut WebSocket,
    current_dir: &str,
    cmd_trimmed: &str,
) -> Option<String> {
    let action = resolve_cd(current_dir, cmd_trimmed);
    match action {
        super::directory::CdAction::Change { new_dir } => {
            let _ = send_msg(socket, ServerMessage::Cd {
                dir: new_dir.clone(),
            })
            .await;
            Some(new_dir)
        }
        super::directory::CdAction::NoOp => None,
    }
}

async fn execute_in_container(
    socket: &mut WebSocket,
    session: &TerminalSession,
    container: &str,
    command: &str,
) {
    match session.run_command(container, command).await {
        Ok(outputs) => {
            send_execution_output(socket, outputs).await;
            let _ = send_msg(socket, ServerMessage::Done).await;
        }
        Err(e) => {
            let _ = send_msg(
                socket,
                ServerMessage::Error {
                    message: e.to_string(),
                },
            )
            .await;
        }
    }
}

async fn send_execution_output(
    socket: &mut WebSocket,
    outputs: Vec<TerminalOutput>,
) {
    for out in outputs {
        match out {
            TerminalOutput::Stdout(data) => {
                let _ = send_msg(socket, ServerMessage::Stdout { data }).await;
            }
            TerminalOutput::Stderr(data) => {
                let _ = send_msg(socket, ServerMessage::Stderr { data }).await;
            }
        }
    }
}
