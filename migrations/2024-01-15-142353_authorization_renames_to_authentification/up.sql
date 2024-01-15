-- Your SQL goes here
-- Rename the table from 'authorizations' to 'authentifications'
ALTER TABLE authorizations RENAME TO authentications;

-- Rename the column from 'type_authorization' to 'type_authentification'
ALTER TABLE authentications RENAME COLUMN type_authorization TO type_authentication;
