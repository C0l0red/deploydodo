-- Add down migration script here
BEGIN;

DROP TABLE servers;
DROP TABLE ssh_keys;
DROP TYPE ServerType;
DROP TYPE AuthType;

COMMIT;
