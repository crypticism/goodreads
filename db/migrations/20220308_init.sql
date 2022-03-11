CREATE TABLE IF NOT EXISTS users (
    id              TEXT    PRIMARY KEY,
    scope           TEXT    NOT NULL,
    access_token    TEXT    NOT NULL,
    token_type      TEXT    NOT NULL,
    profile_id      TEXT,
    title           TEXT,
    update_picture  BOOLEAN NOT NULL DEFAULT false,
    update_status   BOOLEAN NOT NULL DEFAULT false,
    update_title    BOOLEAN NOT NULL DEFAULT false
);