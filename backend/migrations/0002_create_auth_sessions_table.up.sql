-- Add up migration script here
CREATE TABLE auth_sessions(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL,
    session_token VARCHAR(200) NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
