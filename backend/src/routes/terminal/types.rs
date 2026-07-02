use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientMessage {
    Run { container: String, cmd: String },
    Ping,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerMessage {
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
