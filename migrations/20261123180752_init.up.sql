-- Add up migration script here
CREATE TABLE IF NOT EXISTS "url"
(
    "id"               SERIAL PRIMARY KEY,
    "full_url"         TEXT UNIQUE NOT NULL,
    "short_url"        VARCHAR(12) NOT NULL,
    "created_at"       TIMESTAMPTZ NOT NULL,
    "disabled"         BOOLEAN     NOT NULL DEFAULT (FALSE),
    "last_modified_at" TIMESTAMPTZ
);