use std::convert::Infallible;

use async_stream::stream;
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
};
use tokio::sync::broadcast::error::RecvError;

use crate::{dependencies::Dependencies, error::AppResult, services::types::JobStatus};
use crate::{error::AppError, extractors::Auth};

#[utoipa::path(
    get,
    path = "/api/jobs/{job_id}/events",
    params(
        ("job_id" = String, Path, description = "Job ID returned by the setup endpoint"),
            ("Authorization" = String, Header, description = "authorization token")
        ),
    responses(
        (status = 200, description = "SSE stream of job progress events"),
    ),
    tag = "jobs"
)]
pub async fn job_events(
    _: Auth,
    State(deps): State<Dependencies>,
    Path(job_id): Path<String>,
) -> AppResult<Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>>> {
    let status = deps.job_service.get_job_status(&job_id).await?;
    if status.is_none() {
        return Err(AppError::JobNotFound);
    }
    let is_already_done = matches!(status, Some(JobStatus::Completed) | Some(JobStatus::Failed));

    let maybe_rx = deps.job_service.subscribe(&job_id);

    let existing = deps.job_service.get_events(&job_id).await?;
    let last_replayed_id = existing.last().map(|e| e.id).unwrap_or(0);

    let s = stream! {
        for event in existing {
            let is_terminal = is_terminal_event(&event.event_type);
            yield Ok::<Event, Infallible>(
                Event::default().event(&event.event_type).data(&event.data),
            );
            if is_terminal {
                return;
            }
        }

        if is_already_done {
            return;
        }

        let mut rx = match maybe_rx {
            Some(rx) => rx,
            None => return,
        };

        loop {
            match rx.recv().await {
                Ok(event) if event.db_id <= last_replayed_id => {
                    continue;
                }
                Ok(event) => {
                    let is_terminal = is_terminal_event(&event.event_type);
                    yield Ok(Event::default().event(&event.event_type).data(&event.data));
                    if is_terminal {
                        break;
                    }
                }
                Err(RecvError::Closed) => break,
                Err(RecvError::Lagged(_)) => continue,
            }
        }
    };

    Ok(Sse::new(s).keep_alive(KeepAlive::default()))
}

#[inline]
fn is_terminal_event(event_type: &str) -> bool {
    matches!(event_type, "complete" | "error")
}
