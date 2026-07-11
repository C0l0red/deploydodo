-- Add down migration script here
ALTER TABLE servers
ALTER COLUMN hostname SET NOT NULL;
