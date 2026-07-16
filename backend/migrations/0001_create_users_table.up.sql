-- Add up migration script here
BEGIN;

CREATE TYPE AccountType AS ENUM('admin', 'member');

CREATE TABLE users (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(80) NOT NULL,
    email VARCHAR(80) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    account_type AccountType NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

COMMIT;
