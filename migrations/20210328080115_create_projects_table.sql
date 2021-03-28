-- Add migration script here
CREATE TABLE projects
(
    id         uuid        NOT NULL,
    PRIMARY KEY (id),
    name       TEXT        NOT NULL UNIQUE,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);
