use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;

use super::session::terminal_init;
use crate::dependencies::Dependencies;
use crate::error::AppResult;
use crate::routes::terminal::{ControlMessage, TerminalParams};
use crate::services::server_service::ServerId;

pub async fn handle_socket(
    mut socket: WebSocket,
    server_id: ServerId,
    params: TerminalParams,
    deps: Dependencies,
) -> AppResult<()> {
    let mut terminal = terminal_init(server_id, params, &deps).await?;

    loop {
        tokio::select! {
            // shell output → browser
            output = terminal.read() => {
                match output {
                    Some(bytes) => {
                        if socket.send(Message::Binary(bytes.into())).await.is_err() {
                            break; // browser hung up
                        }
                    }
                    None => break, // shell exited / channel closed
                }
            }
            // browser → shell: binary = raw input, text = JSON control message
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(Message::Binary(bytes))) => terminal.write(bytes.as_ref()).await?,
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ControlMessage>(&text) {
                            Ok(ControlMessage::Resize { cols, rows }) => {
                                terminal.resize(cols, rows).await?;
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "ignoring malformed control message");
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
        }
    }

    let _ = socket.close().await;
    Ok(())
}
