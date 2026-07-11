-- Add down migration script here
ALTER TABLE users
ALTER COLUMN created_at TYPE TEXT USING created_at::TEXT;

ALTER TABLE auth_sessions
ALTER COLUMN created_at TYPE TEXT USING created_at::TEXT;

ALTER TABLE variables
ALTER COLUMN created_at TYPE TEXT USING created_at::TEXT;

ALTER TABLE servers
ALTER COLUMN created_at TYPE TEXT USING created_at::TEXT;

ALTER TABLE ssh_keys
ALTER COLUMN created_at TYPE TEXT USING created_at::TEXT;

ALTER TABLE job_events
ALTER COLUMN created_at TYPE TEXT USING created_at::TEXT;

ALTER TABLE jobs
ALTER COLUMN created_at TYPE TEXT USING created_at::TEXT;
ALTER TABLE jobs
ALTER COLUMN updated_at TYPE TEXT USING updated_at::TEXT;
