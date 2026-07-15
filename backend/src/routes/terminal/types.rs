use dodosh::terminal::TermSize;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TerminalParams {
    pub cols: u32,
    pub rows: u32,
    pub container_name: Option<String>,
}

impl From<TerminalParams> for TermSize {
    fn from(value: TerminalParams) -> Self {
        Self::dims(value.cols, value.rows)
    }
}

/// Out-of-band control messages sent by the client over WebSocket **text**
/// frames. Raw keystroke input is sent over binary frames, so control messages
/// never collide with terminal input.
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ControlMessage {
    Resize { cols: u32, rows: u32 },
}
