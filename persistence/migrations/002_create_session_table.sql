CREATE TABLE sessions (
    id BIGSERIAL PRIMARY KEY UNIQUE,
    user_id BIGINT NOT NULL,
    value VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX id_sessions_value ON sessions(id);
CREATE INDEX value_sessions_value ON sessions(value);