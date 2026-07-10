-- Add down migration script here
-- Add up migration script here
ALTER TABLE servers
ALTER COLUMN hostname SET NOT NULL;
