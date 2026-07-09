use pty_process::{Command, OwnedReadPty, OwnedWritePty, Pty, Size};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use crate::{
    terminal::{self, TermSize},
    ShellError,
};

pub struct LocalChannel {
    reader: OwnedReadPty,
    writer: Mutex<OwnedWritePty>,
    child: tokio::process::Child,
}

impl Drop for LocalChannel {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

pub async fn connect_local(size: TermSize) -> Result<terminal::Terminal, ShellError> {
    let pty = Pty::new()?;
    pty.resize(Size::new(size.rows as u16, size.cols as u16))?;

    let pts = pty.pts()?;
    let child = Command::new("bash").spawn(&pts)?;

    let (reader, writer) = pty.into_split();
    Ok(terminal::Terminal::Local(LocalChannel {
        reader,
        writer: Mutex::new(writer),
        child,
    }))
}

impl LocalChannel {
    pub async fn read(&mut self) -> Option<Vec<u8>> {
        let mut buf = [0u8; 4096];
        match self.reader.read(&mut buf).await {
            Ok(0) | Err(_) => None,
            Ok(n) => Some(buf[..n].to_vec()),
        }
    }

    pub async fn write(&self, input: &[u8]) -> Result<(), ShellError> {
        self.writer.lock().await.write_all(input).await?;
        Ok(())
    }

    pub async fn resize(&self, cols: u32, rows: u32) -> Result<(), ShellError> {
        self.writer
            .lock()
            .await
            .resize(Size::new(rows as u16, cols as u16))?;
        Ok(())
    }
}
