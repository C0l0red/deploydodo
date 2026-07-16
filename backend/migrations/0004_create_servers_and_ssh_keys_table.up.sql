-- Add up migration script here
BEGIN;

CREATE TYPE AuthType AS ENUM('keypair', 'password');

CREATE TABLE ssh_keys(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    username VARCHAR(50) NOT NULL,
    password TEXT,
    public_key TEXT,
    private_key TEXT,
    auth_type AuthType NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TYPE ServerType AS ENUM('local', 'remote');

CREATE TABLE servers(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    server_type ServerType NOT NULL,
    hostname VARCHAR(70),
    ssh_port INTEGER,
    ssh_key_id BIGINT,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (ssh_key_id) REFERENCES ssh_keys(id)
);

COMMIT;
