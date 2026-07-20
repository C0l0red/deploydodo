mod handler;
mod session;
mod types;

pub use types::{ControlMessage, TerminalParams};

use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;

use crate::dependencies::Dependencies;
use crate::extractors::Auth;
use crate::services::server_service::ServerId;

pub async fn terminal_ws(
    _: Auth,
    ws: WebSocketUpgrade,
    Query(params): Query<TerminalParams>,
    State(deps): State<Dependencies>,
    Path(server_id): Path<ServerId>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        if let Err(e) = handler::handle_socket(socket, server_id, params, deps).await {
            tracing::error!(error = %e, "terminal ws error");
        }
    })
}
