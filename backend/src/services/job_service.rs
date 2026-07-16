use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use tokio::sync::broadcast;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppResult;

#[derive(Clone, Debug)]
pub struct BroadcastEvent {
    pub db_id: i64,
    pub event_type: String,
    pub data: String,
}

pub struct StoredEvent {
    pub id: i64,
    pub event_type: String,
    pub data: String,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "jobtype", rename_all = "snake_case")]
pub enum JobType {
    CreateServer,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "jobstatus", rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Completed,
    Failed,
}

type SenderMap = Mutex<HashMap<String, broadcast::Sender<BroadcastEvent>>>;

pub struct JobService {
    db: Arc<PgPool>,
    senders: SenderMap,
}

impl JobService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self {
            db,
            senders: Mutex::new(HashMap::new()),
        }
    }

    pub async fn create_job(&self, job_type: JobType) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query!(
            "INSERT INTO jobs (id, job_type, status, created_at, updated_at) \
             VALUES ($1, $2, 'pending', $3, $4)",
            &id,
            job_type as _,
            now,
            now
        )
        .execute(&*self.db)
        .await?;

        let (tx, _) = broadcast::channel(128);
        self.senders.lock().unwrap().insert(id.clone(), tx);

        Ok(id)
    }

    /// Persist an event to the database and broadcast it to all current subscribers.
    pub async fn emit(&self, job_id: &str, event_type: &str, data: Value) -> AppResult<()> {
        let data_str = data.to_string();
        let now = Utc::now();

        let db_id: i64 = sqlx::query_scalar!(
            "INSERT INTO job_events (job_id, event_type, data, created_at) \
             VALUES ($1, $2, $3, $4) RETURNING id",
            job_id,
            event_type,
            &data_str,
            now
        )
        .fetch_one(&*self.db)
        .await?;

        let event = BroadcastEvent {
            db_id,
            event_type: event_type.to_string(),
            data: data_str,
        };

        // Ignore SendError — it just means there are no live subscribers yet.
        if let Some(tx) = self.senders.lock().unwrap().get(job_id) {
            let _ = tx.send(event);
        }

        Ok(())
    }

    /// Mark the job as finished and drop its broadcast sender, closing the channel
    /// for all subscribers.
    pub async fn finish_job(&self, job_id: &str, status: JobStatus) -> AppResult<()> {
        let now = Utc::now();

        sqlx::query!(
            "UPDATE jobs SET status = $1, updated_at = $2 WHERE id = $3",
            status as _,
            now,
            job_id
        )
        .execute(&*self.db)
        .await?;

        // Dropping the sender signals `RecvError::Closed` to all receivers.
        self.senders.lock().unwrap().remove(job_id);

        Ok(())
    }

    /// Load all persisted events for a job in insertion order.
    pub async fn get_events(&self, job_id: &str) -> AppResult<Vec<StoredEvent>> {
        let events: Vec<StoredEvent> = sqlx::query_as!(
            StoredEvent,
            "SELECT id, event_type, data FROM job_events \
             WHERE job_id = $1 ORDER BY id ASC",
            job_id
        )
        .fetch_all(&*self.db)
        .await?;

        Ok(events)
    }

    /// Subscribe to live events for a job.
    /// Returns `None` if the job has already finished (sender already dropped).
    pub fn subscribe(&self, job_id: &str) -> Option<broadcast::Receiver<BroadcastEvent>> {
        self.senders
            .lock()
            .unwrap()
            .get(job_id)
            .map(|tx| tx.subscribe())
    }

    /// Fetch the current status string for a job, or `None` if not found.
    pub async fn get_job_status(&self, job_id: &str) -> AppResult<Option<JobStatus>> {
        Ok(sqlx::query_scalar!(
            r#"SELECT status AS "status!: JobStatus" FROM jobs WHERE id = $1"#,
            job_id
        )
        .fetch_optional(&*self.db)
        .await?)
    }
}
