-- Your SQL goes here
CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    website_link VARCHAR,
    profile_url VARCHAR
);
