-- Add up migration script here
CREATE TABLE ssh_keys(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT,
    public_key TEXT,
    private_key TEXT,
    auth_type TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE servers(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    server_type TEXT NOT NULL,
    hostname TEXT NOT NULL,
    ssh_port INTEGER,
    ssh_key_id INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (ssh_key_id) REFERENCES ssh_keys(id)
);
