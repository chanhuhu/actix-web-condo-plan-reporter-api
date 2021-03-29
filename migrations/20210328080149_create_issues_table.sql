-- Add migration script here
CREATE TABLE issues
(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    floor_plan_id uuid        NOT NULL REFERENCES floor_plans (id),
    name          TEXT        NOT NULL,
    description   TEXT        NOT NULL,
    location      TEXT        NOT NULL,
    created_at    timestamptz NOT NULL,
    updated_at    timestamptz NOT NULL
);
