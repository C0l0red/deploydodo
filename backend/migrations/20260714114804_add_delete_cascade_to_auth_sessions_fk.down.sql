-- Add down migration script here
BEGIN;

ALTER TABLE auth_sessions
  DROP CONSTRAINT auth_sessions_user_id_fkey;

ALTER TABLE auth_sessions
  ADD CONSTRAINT auth_sessions_user_id_fkey
  FOREIGN KEY (user_id)
  REFERENCES users (id);

COMMIT;
