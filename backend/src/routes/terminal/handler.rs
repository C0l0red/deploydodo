use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;

use super::session::terminal_init;
use crate::dependencies::Dependencies;
use crate::error::AppResult;
use crate::routes::terminal::TerminalParams;

pub async fn handle_socket(
    mut socket: WebSocket,
    server_id: i64,
    params: TerminalParams,
    deps: Dependencies,
) -> AppResult<()> {
    let terminal = terminal_init(server_id, params, &deps).await?;

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => terminal.write(text.as_bytes()).await?,
            Message::Binary(bytes) => terminal.write(bytes.iter().as_slice()).await?,
            Message::Close(_) => break,
            _ => {}
        }
    }

    let _ = socket.close().await;
    Ok(())
}
