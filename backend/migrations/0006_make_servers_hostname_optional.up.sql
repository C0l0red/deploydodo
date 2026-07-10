-- Add up migration script here
ALTER TABLE servers
ALTER COLUMN hostname DROP NOT NULL;
