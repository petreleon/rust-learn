CREATE TABLE db_version_control (
    id SERIAL PRIMARY KEY,
    version INTEGER NOT NULL
);

INSERT INTO db_version_control (version) VALUES (0);
