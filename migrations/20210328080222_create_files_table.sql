-- Add migration script here
CREATE TABLE files
(
    id         uuid        NOT NULL,
    PRIMARY KEY (id),
    issue_id   uuid        NOT NULL REFERENCES issues (id),
    name       TEXT        NOT NULL,
    size       BIGINT      NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);
