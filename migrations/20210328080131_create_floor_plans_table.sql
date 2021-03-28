-- Add migration script here
CREATE TABLE floor_plans
(
    id         uuid        NOT NULL,
    PRIMARY KEY (id),
    project_id uuid        NOT NULL REFERENCES projects (id),
    name       TEXT        NOT NULL,
    image_url  TEXT        NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);
