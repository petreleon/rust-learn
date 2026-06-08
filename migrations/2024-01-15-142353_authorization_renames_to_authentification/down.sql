-- This file should undo anything in `up.sql`
-- Revert the table name back to 'authorizations'
ALTER TABLE authentications RENAME TO authorizations;

-- Revert the column name back to 'type_authorization'
ALTER TABLE authorizations RENAME COLUMN type_authentication TO type_authorization;
