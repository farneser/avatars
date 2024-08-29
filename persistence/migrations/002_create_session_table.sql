CREATE TABLE sessions (
    id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL,
    value VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(username)
);

CREATE SEQUENCE sessions_id_seq
    START WITH 1
    INCREMENT BY 1;

CREATE INDEX idx_sessions_value ON sessions(value);