-- Add up migration script here
ALTER TABLE servers
ALTER COLUMN ssh_key_id TYPE BIGINT USING ssh_key_id::BIGINT;
