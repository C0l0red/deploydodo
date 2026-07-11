-- Add down migration script here
ALTER TABLE servers
ALTER COLUMN ssh_key_id TYPE INTEGER USING ssh_key_id::INTEGER;
