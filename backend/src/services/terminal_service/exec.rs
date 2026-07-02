use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::Docker;
use futures_util::StreamExt;

use super::types::TerminalOutput;
use crate::error::AppError;

pub(crate) async fn do_run_command(
    docker: &Docker,
    container_id: &str,
    command: &str,
) -> Result<Vec<TerminalOutput>, AppError> {
    let exec = docker
        .create_exec(
            container_id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["sh".into(), "-c".into(), command.to_string()]),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| AppError::DockerOperation(e.to_string()))?;

    let result = docker
        .start_exec(&exec.id, None::<StartExecOptions>)
        .await
        .map_err(|e| AppError::DockerOperation(e.to_string()))?;

    collect_exec_output(result).await
}

async fn collect_exec_output(
    result: StartExecResults,
) -> Result<Vec<TerminalOutput>, AppError> {
    let mut output = Vec::new();

    if let StartExecResults::Attached {
        output: mut stream, ..
    } = result
    {
        while let Some(item) = stream.next().await {
            match item {
                Ok(bollard::container::LogOutput::StdOut { message })
                | Ok(bollard::container::LogOutput::Console { message }) => {
                    output.push(TerminalOutput::Stdout(
                        String::from_utf8_lossy(&message).to_string(),
                    ));
                }
                Ok(bollard::container::LogOutput::StdErr { message }) => {
                    output.push(TerminalOutput::Stderr(
                        String::from_utf8_lossy(&message).to_string(),
                    ));
                }
                Ok(_) => {}
                Err(e) => {
                    return Err(AppError::DockerOperation(e.to_string()));
                }
            }
        }
    }

    Ok(output)
}
