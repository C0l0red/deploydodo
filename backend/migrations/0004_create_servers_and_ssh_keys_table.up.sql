-- Add up migration script here
CREATE TABLE ssh_keys(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT,
    public_key TEXT,
    private_key TEXT,
    auth_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE servers(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    server_type TEXT NOT NULL,
    hostname TEXT,
    ssh_port INTEGER,
    ssh_key_id BIGINT,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (ssh_key_id) REFERENCES ssh_keys(id)
);
