use axum::extract::ws::{Message, WebSocket};

use super::types::ServerMessage;

pub async fn send_msg(
    socket: &mut WebSocket,
    msg: ServerMessage,
) -> Result<(), crate::error::AppError> {
    let json = serde_json::to_string(&msg).unwrap_or_default();
    socket
        .send(Message::Text(json.into()))
        .await
        .map_err(|e| crate::error::AppError::InternalServerError(e.to_string()))
}
