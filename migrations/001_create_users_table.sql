CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    register_date TIMESTAMPTZ NOT NULL,
    last_update_date TIMESTAMPTZ NOT NULL
);
