-- Your SQL goes here
CREATE TABLE authorizations (
    id SERIAL PRIMARY KEY,
    id_user INT4 REFERENCES users(id),
    type_authorization VARCHAR NOT NULL,
    info_auth TEXT
);
