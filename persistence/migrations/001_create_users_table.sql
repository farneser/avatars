CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY UNIQUE,
    username VARCHAR(255) NOT NULL UNIQUE,
    login_attempts SMALLINT NOT NULL DEFAULT 0,
    register_complete BOOLEAN NOT NULL DEFAULT FALSE,
    primary_email_id BIGINT,
    register_date TIMESTAMPTZ NOT NULL,
    last_update_date TIMESTAMPTZ NOT NULL,
    last_login_date TIMESTAMPTZ
);

CREATE INDEX id_users_username ON users(username);