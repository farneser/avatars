CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    login_attempts SMALLINT NOT NULL DEFAULT 0,
    register_complete BOOLEAN NOT NULL DEFAULT FALSE,
    primary_email_id BIGINT,
    register_date TIMESTAMPTZ NOT NULL,
    last_update_date TIMESTAMPTZ NOT NULL,
    last_login_date TIMESTAMPTZ
);

CREATE SEQUENCE users_id_seq
    START WITH 1
    INCREMENT BY 1;

CREATE INDEX idx_users_username ON users(username);