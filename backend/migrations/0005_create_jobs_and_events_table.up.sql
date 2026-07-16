BEGIN;

CREATE TYPE JobType AS ENUM('create_server');

CREATE TYPE JobStatus AS ENUM('pending', 'completed', 'failed');

CREATE TABLE jobs (
    id VARCHAR(40) PRIMARY KEY,
    job_type JobType NOT NULL,
    status JobStatus NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE job_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    job_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    data TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

COMMIT;
