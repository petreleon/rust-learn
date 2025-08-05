-- Your SQL goes here
CREATE TABLE persistent_states (
    id SERIAL PRIMARY KEY,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL
);
