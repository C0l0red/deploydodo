use dodosh::terminal::TermSize;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TerminalParams {
    pub cols: u32,
    pub rows: u32,
    pub token: String,
}

impl From<TerminalParams> for TermSize {
    fn from(value: TerminalParams) -> Self {
        Self::dims(value.cols, value.rows)
    }
}
