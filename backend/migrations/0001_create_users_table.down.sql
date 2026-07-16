-- Add down migration script here
BEGIN;

DROP TABLE users;
DROP TYPE AccountType;

COMMIT;
