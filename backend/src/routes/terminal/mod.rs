mod handler;
mod session;
mod types;

pub use types::{ControlMessage, TerminalParams};

use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;

use crate::dependencies::Dependencies;

pub async fn terminal_ws(
    ws: WebSocketUpgrade,
    Query(params): Query<TerminalParams>,
    State(deps): State<Dependencies>,
    Path(server_id): Path<i64>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        if let Err(e) = handler::handle_socket(socket, server_id, params, deps).await {
            tracing::error!(error = %e, "terminal ws error");
        }
    })
}
